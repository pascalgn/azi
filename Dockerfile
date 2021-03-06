# Build image
FROM ekidd/rust-musl-builder:1.51.0 AS build

COPY Cargo.toml Cargo.lock /tmp/
COPY src/ /tmp/src/

RUN cargo install --path /tmp && strip /home/rust/.cargo/bin/azi

# Runtime image
FROM alpine:3.13

RUN apk add --no-cache ca-certificates tini && adduser -D -g azi azi

COPY --from=build /home/rust/.cargo/bin/azi /usr/bin/azi

USER azi
WORKDIR /home/azi
ENTRYPOINT [ "/sbin/tini", "--", "/usr/bin/azi" ]
CMD []
