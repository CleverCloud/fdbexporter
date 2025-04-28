ARG FDB_VERSION=7.1.37
ARG RUST_VERSION=1.86.0
# Build Stage
FROM rust:${RUST_VERSION}-bullseye as builder

WORKDIR /app

# Copy only the necessary files for dependency resolution
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the rest of the source code
COPY src ./src

# Build the Rust project
RUN cargo build --release

# Final Stage
FROM debian:bullseye
ARG FDB_VERSION

RUN apt update && apt install -y wget curl dnsutils

WORKDIR /tmp

RUN wget "https://github.com/apple/foundationdb/releases/download/${FDB_VERSION}/foundationdb-clients_${FDB_VERSION}-1_amd64.deb"
RUN dpkg -i foundationdb-clients_${FDB_VERSION}-1_amd64.deb

WORKDIR /app

# Copy the built artifact from the build stage
COPY --from=builder /app/target/release/fdbexporter .
ADD .github/docker/run.sh /app/docker_entrypoint.sh

# Set the command to run on container start
ENTRYPOINT ["/app/docker_entrypoint.sh"]
