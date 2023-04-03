# Stage 1: Build the application
FROM rust:latest AS build

# Set the working directory
WORKDIR /usr/src/app

# Copy the entire source code and build the application
COPY Cargo.toml Cargo.lock ./
COPY entity/ ./entity/
COPY server/ ./server
COPY migration/ ./migration
RUN cargo build --release

RUN cp target/release/migration /usr/src/app/executableM
RUN cp target/release/server /usr/src/app/executableS
RUN rm -rf target


CMD ./executableM; ./executableS
