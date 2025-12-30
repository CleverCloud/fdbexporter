use crate::status_models::cluster_data::ClusterData;
use crate::status_models::cluster_machine::{ClusterMachine, MachineId};
use serde::Deserialize;
use std::collections::HashMap;

use super::cluster_backup::ClusterBackup;
use super::cluster_probe::ClusterLatencyProbe;
use super::cluster_process::{ClusterClassType, ClusterProcess, ProcessId};
use super::cluster_qos::ClusterQos;
use super::cluster_wiggle::ClusterStorageWiggle;

/// jq: .cluster
#[derive(Deserialize)]
pub struct ClusterStatus {
    #[serde(default)]
    pub database_available: bool,
    #[serde(default)]
    pub machines: HashMap<MachineId, ClusterMachine>,
    pub data: Option<ClusterData>,
    #[serde(default)]
    pub processes: HashMap<ProcessId, ClusterProcess>,
    pub latency_probe: Option<ClusterLatencyProbe>,
    #[serde(default)]
    pub generation: i64,
    pub qos: Option<ClusterQos>,
    pub storage_wiggler: Option<ClusterStorageWiggle>,
    pub layers: Option<ClusterStatusLayers>,
}

/// jq: .cluster.layers
#[derive(Deserialize)]
pub struct ClusterStatusLayers {
    #[serde(rename = "_valid")]
    pub valid: bool,
    pub error: Option<String>,

    pub backup: Option<ClusterBackup>,
}

impl ClusterStatus {
    /// Navigate through all process available and their roles
    /// and determine the number of process allocated to a role
    pub fn cluster_roles_count(&self) -> HashMap<ClusterClassType, u8> {
        let mut output: HashMap<ClusterClassType, u8> = HashMap::new();
        let processes_roles = self.processes.values().flat_map(|v| &v.roles);
        for process_role in processes_roles {
            if process_role.role.is_none() {
                continue;
            }
            let role = process_role.role.as_ref().unwrap();
            output.entry(*role).and_modify(|e| *e += 1).or_insert(1);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        net::{Ipv4Addr, SocketAddrV4},
    };

    use either::Left;

    use crate::status_models::{
        address::Address, cluster_data::ClusterData, cluster_process::{ClusterClassType, ClusterProcess, ProcessId}, cluster_process_role::ClusterProcessRole, cluster_qos::ClusterQos
    };

    use super::ClusterStatus;

    impl Default for ClusterProcess {
        fn default() -> Self {
            ClusterProcess {
                address: Address {
                    address: Left(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 65535)),
                    tls: false
                },
                class_source: None,
                class_type: None,
                version: None,
                machine_id: None,
                excluded: None,
                fault_domain: None,
                memory: None,
                network: None,
                run_loop_busy: None,
                uptime_seconds: None,
                cpu: None,
                disk: None,
                roles: Vec::new(),
            }
        }
    }

    impl Default for ClusterStatus {
        fn default() -> Self {
            ClusterStatus {
                database_available: true,
                machines: HashMap::new(),
                data: Some(ClusterData::default()),
                processes: HashMap::new(),
                latency_probe: None,
                generation: 1,
                qos: Some(ClusterQos::default()),
                layers: Some(super::ClusterStatusLayers {
                    valid: true,
                    error: None,
                    backup: None,
                }),
                storage_wiggler: None,
            }
        }
    }

    fn create_process_with_roles(roles: Vec<ClusterClassType>) -> ClusterProcess {
        let process_roles = roles
            .into_iter()
            .map(|r| ClusterProcessRole {
                role: Some(r),
                ..Default::default()
            })
            .collect();
        ClusterProcess {
            roles: process_roles,
            ..Default::default()
        }
    }
    #[test]
    fn count_roles_empty() {
        let status = ClusterStatus::default();
        let count = status.cluster_roles_count();
        assert_eq!(count.len(), 0);
    }

    #[test]
    fn count_roles() {
        let processes = HashMap::from([
            (
                ProcessId("first".to_string()),
                create_process_with_roles(
                    [ClusterClassType::Coordinator, ClusterClassType::Log].into(),
                ),
            ),
            (
                ProcessId("second".to_string()),
                create_process_with_roles(
                    [ClusterClassType::Storage, ClusterClassType::CommitProxy].into(),
                ),
            ),
            (
                ProcessId("third".to_string()),
                create_process_with_roles(
                    [ClusterClassType::Storage, ClusterClassType::Stateless].into(),
                ),
            ),
        ]);
        let status = ClusterStatus {
            processes,
            ..Default::default()
        };
        let count = status.cluster_roles_count();

        assert_eq!(
            count
                .get(&ClusterClassType::Coordinator)
                .unwrap()
                .to_owned(),
            1
        );
        assert_eq!(count.get(&ClusterClassType::Storage).unwrap().to_owned(), 2);
        assert_eq!(
            count.get(&ClusterClassType::Stateless).unwrap().to_owned(),
            1
        );
        assert_eq!(
            count
                .get(&ClusterClassType::CommitProxy)
                .unwrap()
                .to_owned(),
            1
        );
        assert_eq!(count.get(&ClusterClassType::Log).unwrap().to_owned(), 1);
    }
}
