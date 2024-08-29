FROM rust:1.80

WORKDIR /usr/src/project

CMD ["cargo", "build", "--release"]
