FROM ekidd/rust-musl-builder:stable AS rust-builder

ADD . .

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM alpine:3.7

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs
RUN apk add --no-cache ca-certificates && update-ca-certificates

COPY --from=rust-builder /home/rust/src/target/x86_64-unknown-linux-musl/release/actix-helloworld /usr/local/bin/actix-helloworld

ENTRYPOINT [ "actix-helloworld" ]