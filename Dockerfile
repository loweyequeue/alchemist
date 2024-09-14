FROM --platform=linux/amd64 rust:1.81.0-alpine3.20 AS base_rust_alpine

WORKDIR /var/src

# RUN apk add curl=8.9.1-r1
# RUN apk add libgcc=14.2.0-r2
# RUN apk add gcc=14.2.0-r2
# RUN apk add musl=1.2.5-r2
RUN apk add musl-dev  #=1.2.5-r2
#
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --profile minimal --default-toolchain=1.80-x86_64-unknown-linux-musl -y

CMD ["sh"]
