use crate::metrics::MetricsConvertible;
use crate::status_models::cluster_machine::ClusterMachine;
use lazy_static::lazy_static;
use prometheus::{register_gauge_vec, register_int_gauge_vec, GaugeVec, IntGaugeVec};

const MACHINE_LABELS: &[&str] = &["machine_id", "datacenter_id", "address"];

lazy_static! {
    static ref P_CLUSTER_MACHINE_EXCLUDED_GAUGE: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_machine_excluded",
        "Machine is being excluded of the cluster",
        MACHINE_LABELS
    )
    .unwrap();
    static ref P_CLUSTER_MACHINE_CONTRIBUTING_WORKERS_GAUGE: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_machine_contributing_workers_count",
        "Number of process workers on the machine",
        MACHINE_LABELS
    )
    .unwrap();
    static ref P_CLUSTER_MACHINE_MEMORY_COMMITTED_BYTES_GAUGE: IntGaugeVec =
        register_int_gauge_vec!(
            "fdb_cluster_machine_memory_committed_bytes",
            "Estimated number of bytes of memory not available on the machine",
            MACHINE_LABELS
        )
        .unwrap();
    static ref P_CLUSTER_MACHINE_MEMORY_FREE_BYTES_GAUGE: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_machine_memory_free_bytes",
        "Estimated number of bytes of memory that are available on the machine without swapping",
        MACHINE_LABELS
    )
    .unwrap();
    static ref P_CLUSTER_MACHINE_MEMORY_TOTAL_BYTES_GAUGE: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_machine_memory_total_bytes",
        "Estimated number of total physical RAM",
        MACHINE_LABELS
    )
    .unwrap();
    static ref P_CLUSTER_MACHINE_NETWORK_MEGABITS_RECEIVED_GAUGE: GaugeVec = register_gauge_vec!(
        "fdb_cluster_machine_network_received_megabits",
        "Received megabits",
        MACHINE_LABELS
    )
    .unwrap();
    static ref P_CLUSTER_MACHINE_NETWORK_MEGABITS_SENT_GAUGE: GaugeVec = register_gauge_vec!(
        "fdb_cluster_machine_network_sent_megabits",
        "Sent megabits",
        MACHINE_LABELS
    )
    .unwrap();
    static ref P_CLUSTER_MACHINE_NETWORK_TCP_RETRANSMITTED_GAUGE: GaugeVec = register_gauge_vec!(
        "fdb_cluster_machine_network_tcp_segment_retransmitted",
        "Number of TCP segments that have been retransmitted",
        MACHINE_LABELS
    )
    .unwrap();
}

impl MetricsConvertible for ClusterMachine {
    fn to_metrics(&self, labels: &[&str]) {
        P_CLUSTER_MACHINE_CONTRIBUTING_WORKERS_GAUGE
            .with_label_values(labels)
            .set(self.contributing_workers as i64);
        P_CLUSTER_MACHINE_EXCLUDED_GAUGE
            .with_label_values(labels)
            .set(self.excluded as i64);

        P_CLUSTER_MACHINE_MEMORY_COMMITTED_BYTES_GAUGE
            .with_label_values(labels)
            .set(self.memory.committed_bytes);
        P_CLUSTER_MACHINE_MEMORY_FREE_BYTES_GAUGE
            .with_label_values(labels)
            .set(self.memory.free_bytes);
        P_CLUSTER_MACHINE_MEMORY_TOTAL_BYTES_GAUGE
            .with_label_values(labels)
            .set(self.memory.total_bytes);

        P_CLUSTER_MACHINE_NETWORK_MEGABITS_SENT_GAUGE
            .with_label_values(labels)
            .set(self.network.megabits_sent.hz);
        P_CLUSTER_MACHINE_NETWORK_MEGABITS_RECEIVED_GAUGE
            .with_label_values(labels)
            .set(self.network.megabits_received.hz);
        P_CLUSTER_MACHINE_NETWORK_TCP_RETRANSMITTED_GAUGE
            .with_label_values(labels)
            .set(self.network.tcp_segments_retransmitted.hz);
    }
}
