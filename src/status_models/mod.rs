use serde::Deserialize;

pub mod client;
pub mod cluster;
pub mod cluster_backup;
pub mod cluster_data;
pub mod cluster_machine;
pub mod cluster_probe;
pub mod cluster_process;
pub mod cluster_process_disk;
pub mod cluster_process_memory;
pub mod cluster_process_network;
pub mod cluster_process_role;
pub mod cluster_qos;
pub mod cluster_wiggle;
pub mod network_address;

#[derive(Deserialize)]
pub struct Status {
    pub client: client::ClientStatus,
    pub cluster: Option<cluster::ClusterStatus>,
}
