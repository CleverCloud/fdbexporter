use std::collections::HashMap;

use lazy_static::lazy_static;
use prometheus::{
    register_gauge, register_int_gauge, register_int_gauge_vec, Gauge, IntGauge, IntGaugeVec,
};
use tracing::warn;

use crate::{
    metrics::MetricsConvertible,
    status_models::cluster_backup::{ClusterBackup, ClusterBackupTag},
};

use super::{AndSetSingle, StaticMetric};

const P_PREFIX: &str = "fdb_cluster_backup";

lazy_static! {
    static ref P_BACKUP_PAUSED: IntGauge = register_int_gauge!(
        format!("{}_paused", P_PREFIX).as_str(),
        "Backup system enabled (0=false)"
    )
    .unwrap();
    static ref P_BACKUP_WORKERS_TOTAL: IntGauge = register_int_gauge!(
        format!("{}_workers_total", P_PREFIX).as_str(),
        "Backup system number of agent in the cluster"
    )
    .unwrap();
    static ref P_BACKUP_WORKERS_RUNNING: IntGauge = register_int_gauge!(
        format!("{}_workers_running", P_PREFIX).as_str(),
        "Backup system number of agent running in the cluster"
    )
    .unwrap();
    static ref P_BACKUP_RECENT_IO_BYTES_PER_SECOND: Gauge = register_gauge!(
        format!("{}_recent_bytes_per_second", P_PREFIX),
        "Rate of bytes sent per second from backup agents"
    )
    .unwrap();
    static ref P_BACKUP_RECENT_IO_BYTES_SENT: IntGauge = register_int_gauge!(
        format!("{}_recent_bytes_sent", P_PREFIX),
        "Total number of bytes sent recently from backup agents"
    )
    .unwrap();
    static ref P_BACKUP_RECENT_REQUESTS_FAILED: IntGauge = register_int_gauge!(
        format!("{}_recent_requests_failed", P_PREFIX),
        "Recent number of requests failed to external storage from backup agents"
    )
    .unwrap();
    static ref P_BACKUP_RECENT_REQUESTS_SUCCESS: IntGauge = register_int_gauge!(
        format!("{}_recent_requests_successful", P_PREFIX),
        "Recent number of requests done to external storage from backup agents"
    )
    .unwrap();
    static ref P_BACKUP_STATUS_TAG: HashMap<String, IntGaugeVec> = ClusterBackupTag::register(
        format!("{}_tag", P_PREFIX).as_str(),
        "Backup tag information"
    );
}

impl MetricsConvertible for ClusterBackup {
    fn to_metrics(&self, _: &[&str]) {
        P_BACKUP_PAUSED.set(self.paused as i64);

        self.total_workers.and_set(&P_BACKUP_WORKERS_TOTAL);
        self.instances_running.and_set(&P_BACKUP_WORKERS_RUNNING);

        if let Some(io) = &self.blob_recent_io {
            P_BACKUP_RECENT_IO_BYTES_SENT.set(io.bytes_sent);
            P_BACKUP_RECENT_IO_BYTES_PER_SECOND.set(io.bytes_per_second);
            P_BACKUP_RECENT_REQUESTS_FAILED.set(io.requests_failed);
            P_BACKUP_RECENT_REQUESTS_SUCCESS.set(io.requests_successful);
        }

        for (tag, backup) in &self.tags {
            backup.set(&P_BACKUP_STATUS_TAG, &[tag.0.as_str()])
        }
    }
}

impl StaticMetric<IntGaugeVec> for ClusterBackupTag {
    fn register(prefix: &str, desc: &str) -> HashMap<String, IntGaugeVec> {
        let stat_name = &[
            "last_restorable_behind_seconds",
            "last_restorable_version",
            "running_backup",
            "running_backup_restorable",
            "range_bytes_written",
            "mutation_log_written_bytes",
        ];
        let mut metrics = HashMap::new();
        for name in stat_name {
            metrics.insert(
                name.to_string(),
                register_int_gauge_vec!(format!("{}_{}", prefix, name), desc, &["tag"],).unwrap(),
            );
        }
        metrics
    }
    fn set(&self, metrics: &HashMap<String, IntGaugeVec>, labels: &[&str]) {
        let stat_name = &[
            "last_restorable_behind_seconds",
            "last_restorable_version",
            "running_backup",
            "running_backup_restorable",
            "range_bytes_written",
            "mutation_log_written_bytes",
        ];
        for name in *stat_name {
            // Safe as we know already the stat names
            let metric = metrics.get(name).unwrap();
            let value: Option<i64> = match name {
                "last_restorable_behind_seconds" => {
                    self.last_restorable_seconds_behind.map(|v| v.ceil() as i64)
                }
                "last_restorable_version" => self.last_restorable_version,
                "running_backup" => Some(self.running_backup as i64),
                "running_backup_restorable" => Some(self.running_backup_is_restorable as i64),
                "range_bytes_written" => Some(self.range_bytes_written),
                "mutation_log_written_bytes" => Some(self.mutation_log_bytes_written),
                // Impossible case
                &_ => {
                    warn!(
                        "ClusterBackupTag::set() went through irregular case for {}",
                        name
                    );
                    None
                }
            };

            if let Some(value_i64) = value {
                metric.with_label_values(labels).set(value_i64);
            }
        }
    }
}
