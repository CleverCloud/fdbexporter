use serde::Deserialize;

/// Generally the host name, human readable name
#[derive(Deserialize, Eq, PartialEq, Hash)]
pub struct MachineId(pub String);

#[derive(Deserialize, Copy, Clone)]
pub struct Frequency {
    pub hz: f64,
}

impl From<Frequency> for f64 {
    fn from(value: Frequency) -> Self {
        value.hz
    }
}

/// jq: .cluster.machines[]
#[derive(Deserialize)]
pub struct ClusterMachine {
    pub machine_id: MachineId,
    pub address: String,
    pub excluded: bool,
    pub datacenter_id: Option<String>,
    pub memory: ClusterMachineMemory,
    pub contributing_workers: u32,
    pub network: ClusterMachineNetwork,
}

/// jq: .cluster.machines[].memory
#[derive(Deserialize)]
pub struct ClusterMachineMemory {
    pub free_bytes: i64,
    pub committed_bytes: i64,
    pub total_bytes: i64,
}

/// jq: .cluster.machines[].network
#[derive(Deserialize)]
pub struct ClusterMachineNetwork {
    pub megabits_sent: Frequency,
    pub megabits_received: Frequency,
    pub tcp_segments_retransmitted: Frequency,
}
