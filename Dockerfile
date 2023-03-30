FROM rust:1.67
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY entity/ ./entity/
COPY server/ ./server
COPY migration/ ./migration
COPY .env ./
RUN cargo build

CMD ["./target/debug/migration"]
CMD ["./target/debug/server"]
