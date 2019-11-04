# azi

[![Build Status Appveyor](https://img.shields.io/appveyor/ci/pascalgn/azi.svg?style=flat-square&label=appveyor)](https://ci.appveyor.com/project/pascalgn/azi)
[![Build Status CircleCI](https://img.shields.io/circleci/project/pascalgn/azi.svg?style=flat-square&label=circleci)](https://circleci.com/gh/pascalgn/azi)
[![Build Status Docker](https://img.shields.io/docker/cloud/build/pascalgn/azi.svg?style=flat-square)](https://hub.docker.com/r/pascalgn/azi/)
[![License](https://img.shields.io/github/license/pascalgn/azi.svg?style=flat-square)](LICENSE)

Show Azure information.

## Installation

You can download binaries from the [latest release](https://github.com/pascalgn/azi/releases/latest).
If you have Cargo installed, you can use `cargo install azi`

You can also use the Docker image: `docker run --rm pascalgn/azi --help`

## Usage

List all subscriptions and resource groups:

```
azi list
```

Show the costs of March 2019:

```
azi costs 201903
```

Show DNS entries and resource groups they point to:

```
azi domains
```

## Docker

To simply run the command, use `docker run --rm pascalgn/azi`.
If you want to keep the authentication tokens between runs, use

```sh
docker volume create azi
docker run --rm --mount source=azi-tmp,target=/home/azi pascalgn/azi list
docker run --rm --mount source=azi-tmp,target=/home/azi pascalgn/azi dns
docker volume rm azi
```

The images are available on [Docker Hub](https://hub.docker.com/r/pascalgn/azi).

## Helpful links

- https://docs.microsoft.com/en-us/rest/api/azure/
- https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code
- https://docs.microsoft.com/en-us/azure/active-directory/develop/v1-protocols-oauth-code

## License

[Apache-2.0](LICENSE)
