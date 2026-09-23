# Contributing guide

## Local FoundationDB cluster

You can run a fully working foundationDB cluster (3 nodes):

```
docker compose up -d
```

## Running the exporter locally

**You need the FoundationDB client library (`libfdb_c`, shipped in the
`foundationdb-clients` package of
[apple/foundationdb](https://github.com/apple/foundationdb/releases)) installed on
your system to run the exporter and the default test suite.** Only
`cargo test --no-default-features` (parser + metrics, no FoundationDB client)
needs nothing installed.

Generate `fdb.cluster` file from copying from `fdbexporter` container:

```
export CONTAINER_ID=$(docker ps | grep "fdbexporter" | awk '{print $1}')
docker cp ${CONTAINER_ID}:/etc/foundationdb/fdb.cluster ./fdb.cluster
```

Run the project and be sure to set the location of the `fdb.cluster` file:

```
export FDB_CLUSTER_FILE="$PWD/fdb.cluster"
cargo run
```

## Running the tests

```
# Full suite: needs the FoundationDB client library, not a running cluster
# (the doctests which talk to a cluster are `no_run`)
cargo test

# Parser + metrics only: nothing to install
cargo test --no-default-features
```

`tests/status_metrics.rs` feeds `tests/data/simple_fdb.json`, a captured
`status json`, to `parse_status` and `process_metrics` and checks the resulting
Prometheus metrics without a FoundationDB cluster. It runs in both modes (one
extra test covers the `fdb`-only error counters when the feature is on) and is
the place to add a check when you add a metric.

## Project layout

- `src/fetcher.rs`: `parse_status` (always available) and `fetch_cluster_status`,
  which reads the `\xff\xff/status/json` key with the FoundationDB client
  (`fdb` feature only).
- `src/status_models/`: serde models of `status json`.
- `src/metrics/`: the `MetricsConvertible` trait and `process_metrics`, with the
  Prometheus implementation in `src/metrics/prometheus/`.
- `src/main.rs`: the exporter binary (`binary` feature).

## Implement a new metric

We want to have a symetric structure between `status_models` and `prometheus` exporter
files. A trait is available for new structs: `MetricsConvertible`. When using it,
you should ensure its method `to_metric()` is called by its upper struct in models.

1. Ensure the metric is not yet available by exploring `src/metrics/prometheus/`
2. Check the status key is available in models `src/status_models`, if not, add
   necessary structs for it
3. Implemenent `MetricsConvertible` (`src/metrics/mod.rs`) on the new struct, or
   update existin.
4. Ensure `to_metrics()` method is called on your new implementation
