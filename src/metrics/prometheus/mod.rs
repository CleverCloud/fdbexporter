use lazy_static::lazy_static;
use prometheus::{register_int_counter, IntCounter, IntGauge};
use std::collections::HashMap;

use super::MetricsConvertible;
use crate::fetcher::FetchError;

pub mod client;
pub mod cluster;
pub mod cluster_backup;
pub mod cluster_data;
pub mod cluster_machines;
pub mod cluster_probe;
pub mod cluster_process;
pub mod cluster_process_disk;
pub mod cluster_process_memory;
pub mod cluster_process_network;
pub mod cluster_process_role;
pub mod cluster_qos;
pub mod cluster_wiggle;

pub const PROCESS_LABELS: &[&str] = &["machine_id", "process_id", "class_type", "address"];

lazy_static! {
    static ref P_FDB_EXPORTER_PARSING_ERROR: IntCounter = register_int_counter! {
        "fdb_exporter_parsing_error_count",
        "Number of parsing errors encountered",
    }
    .unwrap();
    static ref P_FDB_EXPORTER_FDB_ERROR: IntCounter = register_int_counter!(
        "fdb_exporter_fdb_error_count",
        "Number of FoundationDB errors"
    )
    .unwrap();
    static ref P_FDB_EXPORTER_FDB_BINDING_ERROR: IntCounter = register_int_counter!(
        "fdb_exporter_fdb_binding_error_count",
        "Number of FoundationDB binding errors"
    )
    .unwrap();
    static ref P_FDB_EXPORTER_STATUS_NOT_FOUND: IntCounter = register_int_counter!(
        "fdb_exporter_status_not_found_count",
        "Number of times the status key was not found"
    )
    .unwrap();
}

impl MetricsConvertible for FetchError {
    fn to_metrics(&self, _: &[&str]) {
        match self {
            FetchError::Fdb(_) => P_FDB_EXPORTER_FDB_ERROR.inc(),
            FetchError::FdbBinding(_) => P_FDB_EXPORTER_FDB_BINDING_ERROR.inc(),
            FetchError::StatusNotFound => P_FDB_EXPORTER_STATUS_NOT_FOUND.inc(),
            FetchError::Parsing(_) => P_FDB_EXPORTER_PARSING_ERROR.inc(),
        };
    }
}

/// Implements methods that should be used to register more than one metric on a type
/// [StaticMetric::register] should only be used in lazy_static to generate metrics
/// [StaticMetric::set] should be used in methods to apply new values
pub trait StaticMetric<T> {
    /// Generate a HashMap matching type T with all necessary values to be exposed
    fn register(prefix: &str, desc: &str) -> HashMap<String, T>;
    /// Apply T metrics on the HashMap
    /// [StaticMetric::register] MUST have been used before calling set
    fn set(&self, metric: &HashMap<String, T>, labels: &[&str]);
}

pub trait AndSet<T> {
    /// Borrow [Self] to update a HashMap of metrics
    fn and_set(&self, metric: &HashMap<String, T>);
    /// Borrow [Self] to update a HashMap of metrics where each
    /// metric will have specific labels
    fn and_set_with_labels(&self, metric: &HashMap<String, T>, labels: &[&str]);
}

pub trait AndSetSingle<T> {
    /// Borrow [Self] and update metric [T]
    fn and_set(&self, metric: &T);
}

impl AndSetSingle<IntGauge> for Option<i64> {
    fn and_set(&self, metric: &IntGauge) {
        if let Some(item) = self {
            metric.set(*item);
        }
    }
}

impl<M, T> AndSet<T> for Option<M>
where
    M: StaticMetric<T>,
{
    fn and_set(&self, metric: &HashMap<String, T>) {
        self.and_set_with_labels(metric, &[]);
    }

    fn and_set_with_labels(&self, metric: &HashMap<String, T>, labels: &[&str]) {
        if let Some(v) = self {
            v.set(metric, labels);
        }
    }
}
