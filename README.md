# FoundationDB Metrics Exporter

A tool which will poll status of your FoundationDB cluster and expose human-readable
metrics for Prometheus. When it is useful, metrics are tagged with appropriate data
to be easily retriveable. This software is used in production at 
[Clever Cloud](https://clever.cloud).

Metrics this exporter exposes are available in **[METRICS.md](./METRICS.md)**.

*Not all metrics from status are yet available, but the ones we use are available.
If you need more metrics, feel free to contribute!*

## Migration Notice

**⚠️ Breaking Change in v2.1.0**: The library now uses the native FoundationDB Rust client instead of `fdbcli`. If you're upgrading from an earlier version, please be aware you'll need
to have FoundationDB client lib installed on your system.

## Getting started

### Docker

*We expect that you have a FoundationDB running and accessible from
the container. You can start with [a sample cluster](#running-with-a-sample-foundationdb-cluster)
to try the exporter.*

```
# Pull exporter version 2.1.2 for FoundtionDB version 7.3.69
docker pull clevercloud/fdbexporter:2.1.2-7.3.69
# Environment variables:
#   FDB_COORDINATOR: DNS name of the coordinator node
#   FDB_COORDINATOR_PORT: Port of the coordinator node process
#   FDB_NETWORKING_MODE: Either container or host, describe docker networking
docker run \
  -e FDB_NETWORKING_MODE=container \
  -e FDB_COORDINATOR=coordinator \
  -e FDB_COORDINATOR_PORT=4500 \
  clevercloud/fdbexporter:2.1.2-7.3.69
```

The exporter images are tagged based on both the exporter version and on
FoundationDB versions. Each new version of the exporter will create a container
tag as follow: `${exporter_version}-${foundationdb_version}`. Our CI will create
tags for latest patch version for FoundationDB `7.3` and `7.1`. We do not create
tag for version `7.2` as it shouldn't be used in production.

### Binary

Go to [releases](https://github.com/CleverCloud/fdbexporter/releases) page
and download the compressed asset matching your system distribution.

```
A monitoring tool for FoundationDB with exporting capabilities for prometheus

Usage: fdbexporter [OPTIONS]

Options:
  -p, --port <PORT>            Listening port of the web server [env: FDB_EXPORTER_PORT=] [default: 9090]
  -c, --cluster <CLUSTER>      Location of fdb.cluster file [env: FDB_CLUSTER_FILE=]
  -d, --delay-sec <DELAY_SEC>  Delay in seconds between two update of the status & metrics [env: FDB_EXPORTER_DELAY=] [default: 15]
  -h, --help                   Print help
  -V, --version                Print version
```

### Running with a sample FoundationDB Cluster

Our docker compose will run a fully functional FoundationDB cluster along with the exporter on port `9090`

```
git clone git@github.com:clevercloud/fdbexporter.git
cd fdbexporter
# Run a FoundationDB cluster with the exporter
docker compose up -d
# Fetch metrics available from the exporter
curl localhost:9090
```

## Build

Rust `1.74.0` at least is required

### Building with Different FoundationDB Versions

The exporter supports multiple FoundationDB versions via Cargo features. By default, it builds for FoundationDB 7.3.

```bash
# Build with default (FoundationDB 7.3)
cargo build --release

# Build for FoundationDB 7.1
cargo build --release --no-default-features --features "binary,fdb-7_1"

# Build for FoundationDB 7.3 explicitly
cargo build --release --no-default-features --features "binary,fdb-7_3"

# Build library only for FoundationDB 7.1
cargo build --lib --no-default-features --features fdb-7_1

# Build library only for FoundationDB 7.3
cargo build --lib --no-default-features --features fdb-7_3

# Run the exporter
./target/release/fdbexporter
```

**Note**: The `fdb-7_1` and `fdb-7_3` features are mutually exclusive. You must select only one version at build time.

## Contributing

We welcome contributions, please see [CONTRIBUTING.md](./CONTRIBUTING.md) for more specifics.
