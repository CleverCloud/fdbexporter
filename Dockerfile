ARG FDB_VERSION=7.3.43
ARG RUST_VERSION=1.91.0
# Build Stage
FROM rust:${RUST_VERSION}-bullseye AS builder

ARG FDB_VERSION

WORKDIR /app

RUN apt update && apt install -y wget libclang-dev libssl-dev pkg-config openssh-client

# Download foundationdb client
RUN wget -q https://github.com/apple/foundationdb/releases/download/${FDB_VERSION}/foundationdb-clients_${FDB_VERSION}-1_amd64.deb
RUN dpkg -i foundationdb-clients_${FDB_VERSION}-1_amd64.deb

COPY Cargo.toml Cargo.lock ./
COPY build.rs ./
COPY src ./src

# Build the project with specific features based on FoundationDB version
RUN if echo "${FDB_VERSION}" | grep -q "^7\.1\."; then \
      CARGO_FEATURES="--no-default-features --features binary,fdb-7_1"; \
    elif echo "${FDB_VERSION}" | grep -q "^7\.3\."; then \
      CARGO_FEATURES="--no-default-features --features binary,fdb-7_3"; \
    else \
      echo "Unsupported FDB_VERSION: ${FDB_VERSION}. Must be 7.1.x or 7.3.x" && exit 1; \
    fi && \
    echo "Building with features: ${CARGO_FEATURES}" && \
    cargo build --release ${CARGO_FEATURES}

# Final Stage
FROM debian:bullseye
ARG FDB_VERSION
WORKDIR /app

RUN apt update && apt install -y wget curl dnsutils openssh-client

# Download foundationdb client
RUN wget -q https://github.com/apple/foundationdb/releases/download/${FDB_VERSION}/foundationdb-clients_${FDB_VERSION}-1_amd64.deb \
    && dpkg -i foundationdb-clients_${FDB_VERSION}-1_amd64.deb

# Copy the built artifact from the build stage
COPY --from=builder /app/target/release/fdbexporter .
ADD .github/docker/run.sh /app/docker_entrypoint.sh

# Set the command to run on container start
ENTRYPOINT ["/app/docker_entrypoint.sh"]
