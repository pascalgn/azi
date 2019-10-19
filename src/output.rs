use colored::Colorize;
use serde_json::to_string;
use serde_json::to_string_pretty;
use serde_json::Value;

use crate::commands::CostResult;
use crate::commands::DnsResult;
use crate::commands::Domain;
use crate::commands::IpResult;
use crate::commands::ListResult;
use crate::object::DnsRecordEntry;
use crate::object::Identifiable;
use crate::utils::Result;

pub trait Output {
    fn print_list_results(&self, results: &Vec<ListResult>, id: bool) -> Result<()>;

    fn print_domains(&self, domains: &Vec<Domain>) -> Result<()>;

    fn print_dns_results(&self, results: &Vec<DnsResult>) -> Result<()>;

    fn print_ip_results(&self, results: &Vec<IpResult>) -> Result<()>;

    fn print_cost_results(&self, results: &Vec<CostResult>) -> Result<()>;

    fn print_value(&self, value: &Value) -> Result<()>;
}

pub struct JsonOutput {}

impl Output for JsonOutput {
    fn print_list_results(&self, results: &Vec<ListResult>, _: bool) -> Result<()> {
        println!("{}", to_string_pretty(results)?);
        return Ok(());
    }

    fn print_domains(&self, domains: &Vec<Domain>) -> Result<()> {
        println!("{}", to_string_pretty(domains)?);
        return Ok(());
    }

    fn print_dns_results(&self, results: &Vec<DnsResult>) -> Result<()> {
        println!("{}", to_string_pretty(results)?);
        return Ok(());
    }

    fn print_ip_results(&self, results: &Vec<IpResult>) -> Result<()> {
        println!("{}", to_string_pretty(results)?);
        return Ok(());
    }

    fn print_cost_results(&self, results: &Vec<CostResult>) -> Result<()> {
        println!("{}", to_string_pretty(results)?);
        return Ok(());
    }

    fn print_value(&self, value: &Value) -> Result<()> {
        println!("{}", to_string_pretty(value)?);
        return Ok(());
    }
}

pub struct TextOutput {}

impl Output for TextOutput {
    fn print_list_results(&self, results: &Vec<ListResult>, id: bool) -> Result<()> {
        for result in results {
            if id {
                println!(
                    "{} {}",
                    result.subscription.name.red(),
                    format!("({})", result.subscription.subscription_id).dimmed()
                );
            } else {
                println!("{}", result.subscription.name.red());
            }

            for resource_group in &result.resource_groups {
                println!("  {}", resource_group.name.blue());

                for resource in &result.resources {
                    if resource.resource_group()? == resource_group.name {
                        if id {
                            println!(
                                "    {} {} {}",
                                resource.name,
                                format!("({})", resource.resource_type).dimmed(),
                                format!("({})", resource.id).dimmed()
                            );
                        } else {
                            println!(
                                "    {} {}",
                                resource.name,
                                format!("({})", resource.resource_type).dimmed()
                            );
                        }
                    }
                }
            }
        }

        return Ok(());
    }

    fn print_domains(&self, domains: &Vec<Domain>) -> Result<()> {
        for domain in domains {
            println!("{}", domain.name.cyan());

            let arrow = "->".dimmed();

            let mut depth = 0;
            for entry in &domain.entries {
                match entry {
                    Some(DnsRecordEntry::CNAME(cname)) => {
                        println!("{0:1$} {2} {3}", "", depth * 4, arrow, cname);
                        depth += 1;
                    }
                    None => println!(
                        "{0:1$} {2} {3}",
                        "",
                        depth * 4,
                        arrow,
                        "[recursion depth exceeded]".red()
                    ),
                    _ => (),
                }
            }

            for ip_address in &domain.ip_addresses {
                println!(
                    "{0:1$} {2} {3}",
                    "",
                    depth * 4,
                    arrow,
                    ip_address.ip_address
                );

                if let Some(resource_group) = ip_address.resource_group.as_ref() {
                    println!(
                        "{0:1$}     {2} {3}",
                        "",
                        depth * 4,
                        arrow,
                        resource_group.name.blue()
                    );
                }
            }
        }

        return Ok(());
    }

    fn print_dns_results(&self, results: &Vec<DnsResult>) -> Result<()> {
        for result in results {
            println!("{}", result.zone.name.blue());

            for record in &result.records {
                println!("  {}", record.name.cyan());
                match &record.entry {
                    DnsRecordEntry::A(ip_addresses) => {
                        for ip in ip_addresses {
                            println!("    {} {}", "A".dimmed(), ip);
                        }
                    }
                    DnsRecordEntry::CNAME(cname) => println!("    {} {}", "CNAME".dimmed(), cname),
                }
            }
        }

        return Ok(());
    }

    fn print_ip_results(&self, results: &Vec<IpResult>) -> Result<()> {
        for result in results {
            println!("{}", result.subscription.name.red());

            for resource_group in &result.resource_groups {
                println!("  {}", resource_group.resource_group.name.blue());

                for ip in &resource_group.ip_addresses {
                    println!("    {}", ip.ip_address);
                }
            }
        }

        return Ok(());
    }

    fn print_cost_results(&self, results: &Vec<CostResult>) -> Result<()> {
        let mut total = 0.0;
        let mut total_currency = None;

        for result in results {
            println!("{}", result.subscription.name.red());

            let mut sum = 0.0;
            let mut sum_currency = None;

            for item in &result.costs {
                println!(
                    "  {}  {:0.2} {}",
                    item.resource_group.blue(),
                    item.costs,
                    item.currency
                );
                sum += item.costs;
                if sum_currency == None {
                    sum_currency = Some(&item.currency);
                }
            }

            if let Some(currency) = sum_currency {
                println!("  {}  {:0.2} {}", "sum".cyan(), sum, currency);
                total += sum;
                total_currency = Some(currency.clone());
            }
        }

        if let Some(currency) = total_currency {
            println!("{}  {:0.2} {}", "total".cyan(), total, currency);
        }

        return Ok(());
    }

    fn print_value(&self, value: &Value) -> Result<()> {
        println!("{}", to_string(value)?);
        return Ok(());
    }
}
