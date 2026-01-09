use serde::Deserialize;

use super::cluster_process::ProcessId;
use super::network_address::NetworkAddress;

/// jq: .cluster.storage_wiggle
#[derive(Deserialize)]
pub struct ClusterStorageWiggle {
    pub primary: Option<ClusterStoragePrimaryWiggle>,

    pub wiggle_server_addresses: Vec<NetworkAddress>,
    pub wiggle_server_ids: Vec<ProcessId>,
}

/// jq: .cluster.storage_wiggle.primary
#[derive(Deserialize)]
pub struct ClusterStoragePrimaryWiggle {
    pub finished_round: u16,
    pub finished_wiggle: u16,

    pub smoothed_round_seconds: f64,
    pub smoothed_wiggle_seconds: f64,

    pub last_round_finish_timestamp: f64,
    pub last_round_start_timestamp: f64,
    pub last_wiggle_finish_timestamp: f64,
    pub last_wiggle_start_timestamp: f64,
}
