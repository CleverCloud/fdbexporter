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

**⚠️ Breaking Change in v2.5.0**: the native FoundationDB Rust client
([fdb-rs](https://github.com/foundationdb-rs/foundationdb-rs), crate `foundationdb`) is an optional
dependency behind the `fdb` cargo feature, enabled by default. The `binary`, `fdb-7_1` and `fdb-7_3`
features imply it, so library users already depending on
`fdbexporter = { ..., default-features = false, features = ["fdb-7_3"] }` keep `fetch_cluster_status`
unchanged. `default-features = false` alone now gives a parser + metrics only crate, with no
`foundationdb*` crate in the dependency graph, see
[Using fdbexporter as a library](#using-fdbexporter-as-a-library). The same release bumps fdb-rs to
`0.11.0`: the payloads of `FetchError::Fdb` and `FetchError::FdbBinding` are fdb-rs `0.11` types, so
code matching on them must use fdb-rs `0.11` too.

## Getting started

### Docker

*We expect that you have a FoundationDB running and accessible from
the container. You can start with [a sample cluster](#running-with-a-sample-foundationdb-cluster)
to try the exporter.*

```
# Pull exporter version 2.4.0 for FoundtionDB version 7.3.77
docker pull clevercloud/fdbexporter:2.4.0-7.3.77
# Environment variables:
#   FDB_COORDINATOR: DNS name of the coordinator node
#   FDB_COORDINATOR_PORT: Port of the coordinator node process
#   FDB_NETWORKING_MODE: Either container or host, describe docker networking
docker run \
  -e FDB_NETWORKING_MODE=container \
  -e FDB_COORDINATOR=coordinator \
  -e FDB_COORDINATOR_PORT=4500 \
  clevercloud/fdbexporter:2.4.0-7.3.77
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
  -p, --port <PORT>                Listening port of the web server [env: FDB_EXPORTER_PORT=] [default: 9090]
  -a, --addr <ADDR>                Listening IPv4/IPv6 address of the web server [env: FDB_EXPORTER_ADDR=] [default: 0.0.0.0]
  -c, --cluster <CLUSTER>          Location of fdb.cluster file [env: FDB_CLUSTER_FILE=]
  -d, --delay-sec <DELAY_SEC>      Delay in seconds between two update of the status & metrics [env: FDB_EXPORTER_DELAY=] [default: 15]
  -t, --fdb-timeout <FDB_TIMEOUT>  Timeout in seconds for FoundationDB status fetch operations [env: FDB_TIMEOUT=] [default: 60]
  -h, --help                       Print help
  -V, --version                    Print version
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

## Using fdbexporter as a library

The crate can be used in two modes, selected with cargo features:

| Feature   | What it enables                                                              | Notes                                                                                    |
| --------- | ---------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| `binary`  | The `fdbexporter` executable: CLI, HTTP server and polling loop              | Implies `fdb`. Default                                                                   |
| `fdb`     | The native FoundationDB client (fdb-rs) and `fetch_cluster_status`           | Must be combined with `fdb-7_1` or `fdb-7_3`: fdb-rs itself rejects `fdb` alone. Default |
| `fdb-7_1` | fdb-rs built against the FoundationDB 7.1 client API                         | Implies `fdb`. Mutually exclusive with `fdb-7_3`                                         |
| `fdb-7_3` | fdb-rs built against the FoundationDB 7.3 client API                         | Implies `fdb`. Mutually exclusive with `fdb-7_1`. Default                                |

Without `fdb`, the crate only contains the `status json` serde models
(`fdbexporter::status_models`), the `parse_status` parser and the Prometheus metrics
(`process_metrics`): no `foundationdb*` crate ends up in your dependency graph. This is
meant for programs which already link fdb-rs through another library and must not carry
a second copy of the client.

### Full mode: fdbexporter fetches the status

```toml
[dependencies]
fdbexporter = { git = "https://github.com/CleverCloud/fdbexporter.git", default-features = false, features = ["fdb-7_3"] }
```

`fetch_cluster_status` reads the `\xff\xff/status/json` system key through the FoundationDB
client library, which must be installed on the machine (`libfdb_c`, from the
`foundationdb-clients` package of [apple/foundationdb](https://github.com/apple/foundationdb/releases)).
The client network must be started once per process with `unsafe { foundationdb::boot() }`
before calling it. See the documentation of `fetch_cluster_status` (`cargo doc --open`) for a
full example.

### Parser-only mode: bring your own status JSON

```toml
[dependencies]
fdbexporter = { git = "https://github.com/CleverCloud/fdbexporter.git", default-features = false }
prometheus = "0.13"
```

Obtain the status JSON bytes yourself, either by reading the `\xff\xff/status/json` key with
your own fdb-rs client (with the `ReadSystemKeys` transaction option) or with
`fdbcli --exec "status json"`, then hand them to the crate:

```rust
use fdbexporter::{parse_status, process_metrics, MetricsConvertible};
use prometheus::{Encoder, TextEncoder};

/// `status_json` is the raw value of the `\xff\xff/status/json` key
fn refresh(status_json: &[u8]) -> Vec<u8> {
    match parse_status(status_json) {
        // Updates the gauges and drops the label sets (processes, machines,
        // coordinators, roles, backup tags) gone since the previous status
        Ok(status) => process_metrics(status),
        // Increments `fdb_exporter_parsing_error_count`
        Err(e) => e.to_metrics(&[]),
    }

    // Everything is registered in the prometheus default registry
    let mut buffer = vec![];
    TextEncoder::new()
        .encode(&prometheus::gather(), &mut buffer)
        .unwrap();
    buffer
}
```

Always go through `process_metrics`: calling `to_metrics` on the models directly skips the
reset of stale label sets, and processes which left the cluster would be exported forever at
their last value. All metrics are registered in the default registry of the `prometheus`
crate, so gather them with the same `prometheus` `0.13` crate the library uses: another
major version would be a distinct crate with its own registry.

Without `fdb`, `FetchError` has no `Fdb` and `FdbBinding` variants. Cargo unifies features
across a build though, so they come back as soon as any crate in your graph enables `fdb`:
do not match on `FetchError` exhaustively in parser-only code.

## Build

Rust `1.85.1` at least is required: fdb-rs `0.11` and its sibling crates declare
`rust-version = "1.85.1"` and use edition 2024.

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

# Build library only, parser + metrics without the FoundationDB client
# (no FoundationDB client library needed)
cargo build --lib --no-default-features

# Run the tests without the FoundationDB client
cargo test --no-default-features

# Run the exporter
./target/release/fdbexporter
```

**Note**: The `fdb-7_1` and `fdb-7_3` features are mutually exclusive. You must select only one version at build time. `fdb` on its own, without a version feature, is rejected by fdb-rs.

## Contributing

We welcome contributions, please see [CONTRIBUTING.md](./CONTRIBUTING.md) for more specifics.
