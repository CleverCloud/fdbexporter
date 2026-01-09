use core::fmt;

use serde::Deserialize;

use super::cluster_machine::MachineId;
use super::cluster_process_disk::ClusterProcessDisk;
use super::cluster_process_memory::ClusterProcessMemory;
use super::cluster_process_network::ClusterProcessNetwork;
use super::cluster_process_role::ClusterProcessRole;
use super::network_address::NetworkAddress;

/// A hash corresponding to the process
#[derive(Deserialize, PartialEq, Eq, Hash)]
pub struct ProcessId(pub String);

/// jq: .cluster.processes[]
#[derive(Deserialize)]
pub struct ClusterProcess {
    pub address: NetworkAddress,
    pub class_source: Option<ClusterClassSource>,
    pub class_type: Option<ClusterClassType>,
    pub version: Option<String>,
    pub machine_id: Option<MachineId>,
    pub excluded: Option<bool>,
    pub fault_domain: Option<String>,
    pub memory: Option<ClusterProcessMemory>,
    pub network: Option<ClusterProcessNetwork>,
    pub run_loop_busy: Option<f64>,
    pub uptime_seconds: Option<f64>,
    pub cpu: Option<ClusterProcessCpu>,
    pub disk: Option<ClusterProcessDisk>,
    pub roles: Vec<ClusterProcessRole>,
}

/// jq: .cluster.processes[].cpu
#[derive(Deserialize)]
pub struct ClusterProcessCpu {
    pub usage_cores: f64,
}

#[derive(Deserialize)]
pub enum ClusterClassSource {
    #[serde(rename = "command_line")]
    CommandLine,
    #[serde(rename = "configure_auto")]
    ConfigureAuto,
    #[serde(rename = "set_class")]
    SetClass,
}

#[derive(Deserialize, Eq, Hash, PartialEq, Clone, Copy)]
pub enum ClusterClassType {
    #[serde(rename = "unset")]
    Unset,
    #[serde(rename = "storage")]
    Storage,
    #[serde(rename = "transaction")]
    Transaction,
    #[serde(rename = "resolution")]
    Resolution,
    #[serde(rename = "stateless")]
    Stateless,
    #[serde(rename = "consistency_scan")]
    ConsistencyScan,
    #[serde(rename = "commit_proxy")]
    CommitProxy,
    #[serde(rename = "grv_proxy")]
    GrvProxy,
    #[serde(rename = "master")]
    Master,
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "storage_cache")]
    StorageCache,
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "cluster_controller")]
    ClusterController,
    #[serde(rename = "data_distributor")]
    DataDistributor,
    #[serde(rename = "ratekeeper")]
    RateKeeper,
    #[serde(rename = "coordinator")]
    Coordinator,
    #[serde(rename = "resolver")]
    Resolver,
}

impl fmt::Display for ClusterClassType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClusterClassType::Unset => write!(f, "unset"),
            ClusterClassType::Storage => write!(f, "storage"),
            ClusterClassType::Transaction => write!(f, "transaction"),
            ClusterClassType::Resolution => write!(f, "resolution"),
            ClusterClassType::Stateless => write!(f, "stateless"),
            ClusterClassType::ConsistencyScan => write!(f, "consistency_scan"),
            ClusterClassType::CommitProxy => write!(f, "commit_proxy"),
            ClusterClassType::GrvProxy => write!(f, "grv_proxy"),
            ClusterClassType::Master => write!(f, "master"),
            ClusterClassType::Test => write!(f, "test"),
            ClusterClassType::StorageCache => write!(f, "storage_cache"),
            ClusterClassType::Log => write!(f, "log"),
            ClusterClassType::ClusterController => write!(f, "cluster_controller"),
            ClusterClassType::DataDistributor => write!(f, "data_distributor"),
            ClusterClassType::RateKeeper => write!(f, "rate_keeper"),
            ClusterClassType::Coordinator => write!(f, "coordinator"),
            ClusterClassType::Resolver => write!(f, "resolver"),
        }
    }
}
