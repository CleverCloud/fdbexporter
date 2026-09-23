//! Parses a real `status json` document and exports it through the public API.
//! Runs with and without the `fdb` feature. The default Prometheus registry is
//! process-global, so each `fdb_exporter_*` counter is exercised by a single test.

use fdbexporter::{parse_status, process_metrics, FetchError, MetricsConvertible};
use prometheus::proto::MetricFamily;

/// `status json` of a three-process FoundationDB 7.1 cluster.
const SIMPLE_FDB_STATUS: &[u8] = include_bytes!("data/simple_fdb.json");

fn family(name: &str) -> Option<MetricFamily> {
    prometheus::gather()
        .into_iter()
        .find(|family| family.get_name() == name)
}

fn registered_family(name: &str) -> MetricFamily {
    family(name).unwrap_or_else(|| panic!("`{name}` is not registered"))
}

fn int_gauge(name: &str) -> i64 {
    registered_family(name).get_metric()[0]
        .get_gauge()
        .get_value() as i64
}

fn labelled_int_gauge(name: &str, label: &str, value: &str) -> i64 {
    let family = registered_family(name);
    let metric = family
        .get_metric()
        .iter()
        .find(|metric| {
            metric
                .get_label()
                .iter()
                .any(|pair| pair.get_name() == label && pair.get_value() == value)
        })
        .unwrap_or_else(|| panic!("`{name}` has no series with {label}=\"{value}\""));
    metric.get_gauge().get_value() as i64
}

fn counter(name: &str) -> u64 {
    family(name).map_or(0, |family| {
        family.get_metric()[0].get_counter().get_value() as u64
    })
}

#[test]
fn simple_status_exports_metrics() {
    let status = parse_status(SIMPLE_FDB_STATUS).expect("the fixture is a valid status");
    process_metrics(status);

    assert_eq!(int_gauge("fdb_client_timestamp"), 1_704_187_851);
    assert_eq!(int_gauge("fdb_client_coordinators_count"), 1);
    assert_eq!(
        labelled_int_gauge(
            "fdb_client_coordinator_reachable",
            "address",
            "172.19.0.2:4500"
        ),
        1
    );
    assert_eq!(int_gauge("fdb_client_quorum_reachable"), 1);
    assert_eq!(int_gauge("fdb_database_available"), 1);
    assert_eq!(int_gauge("fdb_database_healthy"), 1);

    assert_eq!(int_gauge("fdb_cluster_machines_count"), 3);
    assert_eq!(int_gauge("fdb_cluster_generation_count"), 2);
    assert_eq!(
        labelled_int_gauge("fdb_cluster_processes_roles", "role", "storage"),
        3
    );
    assert_eq!(int_gauge("fdb_cluster_partition_count"), 1);
    assert_eq!(int_gauge("fdb_cluster_healthy"), 1);
    assert_eq!(
        registered_family("fdb_cluster_process_uptime")
            .get_metric()
            .len(),
        3
    );
}

#[test]
fn invalid_status_is_a_parsing_error() {
    let Err(err) = parse_status(b"{}") else {
        panic!("an empty object is not a valid status");
    };
    assert!(
        matches!(err, FetchError::Parsing(_)),
        "unexpected error: {err:?}"
    );

    let before = counter("fdb_exporter_parsing_error_count");
    err.to_metrics(&[]);
    assert_eq!(counter("fdb_exporter_parsing_error_count"), before + 1);
}

#[test]
fn status_not_found_is_counted() {
    let before = counter("fdb_exporter_status_not_found_count");
    FetchError::StatusNotFound.to_metrics(&[]);
    assert_eq!(counter("fdb_exporter_status_not_found_count"), before + 1);
}

#[cfg(feature = "fdb")]
#[test]
fn fdb_errors_are_counted() {
    use foundationdb::{FdbBindingError, FdbError};

    let before = counter("fdb_exporter_fdb_error_count");
    // 1031: transaction_timed_out
    FetchError::from(FdbError::from_code(1031)).to_metrics(&[]);
    assert_eq!(counter("fdb_exporter_fdb_error_count"), before + 1);

    let before = counter("fdb_exporter_fdb_binding_error_count");
    let custom = Box::new(std::io::Error::other("invalid cluster file path"));
    FetchError::from(FdbBindingError::CustomError(custom)).to_metrics(&[]);
    assert_eq!(counter("fdb_exporter_fdb_binding_error_count"), before + 1);
}
