use crate::metrics::MetricsConvertible;
use crate::status_models::cluster_data::ClusterData;
use lazy_static::lazy_static;
use prometheus::{register_int_gauge, IntGauge};

lazy_static! {
    static ref P_CLUSTER_AVG_PARTITION_BYTES_GAUGE: IntGauge = register_int_gauge!(
        "fdb_cluster_average_partition_size_bytes",
        "Average size for a partition in the cluster"
    )
    .unwrap();
    static ref P_CLUSTER_LEAST_SPACE_BYTES_LOG_SERVER_GAUGE: IntGauge = register_int_gauge!(
        "fdb_cluster_least_space_log_server_bytes",
        "Value of the log server with least space available"
    )
    .unwrap();
    static ref P_CLUSTER_LEAST_SPACE_BYTES_STORAGE_SERVER: IntGauge = register_int_gauge!(
        "fdb_cluster_least_space_storage_server_bytes",
        "Value of the storage server with least space avaiable"
    )
    .unwrap();
    static ref P_CLUSTER_PARTITION_COUNT: IntGauge =
        register_int_gauge!("fdb_cluster_partition_count", "Number of partitions").unwrap();
    static ref P_CLUSTER_TOTAL_DISK_USED_BYTES: IntGauge = register_int_gauge!(
        "fdb_cluster_total_disk_used_bytes",
        "Total number of bytes used on all disk"
    )
    .unwrap();
    static ref P_CLUSTER_TOTAL_KV_SIZE_BYTES: IntGauge = register_int_gauge!(
        "fdb_cluster_total_kv_size_bytes",
        "Total number of bytes for all key values"
    )
    .unwrap();
    static ref P_CLUSTER_STATE_HEALTHY: IntGauge = register_int_gauge!(
        "fdb_cluster_healthy",
        "Whether the cluster is healthy or not"
    )
    .unwrap();
    static ref P_CLUSTER_STATE_CURRENT: IntGauge = register_int_gauge!(
        "fdb_cluster_state",
        "Current state of the cluster (see src/status_models/cluster_data.rs)"
    )
    .unwrap();
    static ref P_CLUSTER_MOVING_DATA_IN_FLIGHT_BYTES: IntGauge =
        register_int_gauge!("fdb_cluster_moving_data_in_flight_bytes", "Data in flight",).unwrap();
    static ref P_CLUSTER_MOVING_DATA_IN_QUEUE_BYTES: IntGauge = register_int_gauge!(
        "fdb_cluster_moving_data_in_queue_bytes",
        "Data waiting to be transferred"
    )
    .unwrap();
}

impl MetricsConvertible for ClusterData {
    fn to_metrics(&self, _: &[&str]) {
        if let Some(total_kv_size_bytes) = self.total_kv_size_bytes {
            P_CLUSTER_TOTAL_KV_SIZE_BYTES.set(total_kv_size_bytes);
        }
        if let Some(total_disk_used_bytes) = self.total_disk_used_bytes {
            P_CLUSTER_TOTAL_DISK_USED_BYTES.set(total_disk_used_bytes);
        }
        if let Some(partitions_count) = self.partitions_count {
            P_CLUSTER_PARTITION_COUNT.set(partitions_count);
        }
        if let Some(least_operating_space_bytes_log_server) =
            self.least_operating_space_bytes_log_server
        {
            P_CLUSTER_LEAST_SPACE_BYTES_LOG_SERVER_GAUGE
                .set(least_operating_space_bytes_log_server);
        }
        if let Some(least_operating_space_bytes_storage_server) =
            self.least_operating_space_bytes_storage_server
        {
            P_CLUSTER_LEAST_SPACE_BYTES_STORAGE_SERVER
                .set(least_operating_space_bytes_storage_server);
        }
        if let Some(average_partition_size_bytes) = self.average_partition_size_bytes {
            P_CLUSTER_AVG_PARTITION_BYTES_GAUGE.set(average_partition_size_bytes);
        }
        if let Some(state) = &self.state {
            if let Some(health) = state.healthy {
                P_CLUSTER_STATE_HEALTHY.set(health as i64);
            }
            P_CLUSTER_STATE_CURRENT.set(state.name as i64);
        }

        if let Some(moving_data) = &self.moving_data {
            P_CLUSTER_MOVING_DATA_IN_FLIGHT_BYTES.set(moving_data.in_flight_bytes);
            P_CLUSTER_MOVING_DATA_IN_QUEUE_BYTES.set(moving_data.in_queue_bytes);
        }
    }
}
