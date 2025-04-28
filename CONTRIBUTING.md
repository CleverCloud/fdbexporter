# Contributing guide

## Local FoundationDB cluster

You can run a fully working foundationDB cluster (3 nodes):

```
docker compose up -d
```

## Running the exporter locally

**You need to have `fdbcli` installed on your system, see
[apple/foundationdb](https://github.com/apple/foundationdb/releases)**

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
