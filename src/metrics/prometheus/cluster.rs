use crate::status_models::cluster::ClusterStatus;
use crate::{metrics::MetricsConvertible, status_models::cluster_process::ClusterClassType};
use lazy_static::lazy_static;
use prometheus::{register_int_gauge, register_int_gauge_vec, IntGauge, IntGaugeVec};

lazy_static! {
    static ref P_CLUSTER_MACHINES_COUNT: IntGauge = register_int_gauge!(
        "fdb_cluster_machines_count",
        "Number of machines available in the cluster"
    )
    .unwrap();
    static ref P_CLUSTER_PROCESS_ROLES_COUNT: IntGaugeVec = register_int_gauge_vec!(
        "fdb_cluster_processes_roles",
        "Current number of process running a specific role",
        &["role"]
    )
    .unwrap();
    static ref P_CLUSTER_GENERATION_COUNT: IntGauge =
        register_int_gauge!("fdb_cluster_generation_count", "Number of generations").unwrap();
}

impl MetricsConvertible for ClusterStatus {
    fn to_metrics(&self, _: &[&str]) {
        P_CLUSTER_MACHINES_COUNT.set(self.machines.len() as i64);

        for (machine_id, machine) in &self.machines {
            let datacenter_id = machine
                .datacenter_id
                .clone()
                .unwrap_or(String::from("default"));
            let labels = [
                machine_id.0.as_str(),
                datacenter_id.as_str(),
                machine.address.as_str(),
            ];
            machine.to_metrics(&labels);
        }

        if let Some(data) = &self.data {
            data.to_metrics(&[]);
        }

        for (process_id, process) in &self.processes {
            let machine_id = match &process.machine_id {
                Some(id) => id,
                None => continue,
            };
            let class_type = process
                .class_type
                .as_ref()
                .unwrap_or(&ClusterClassType::Unset)
                .to_string();
            let labels = [
                machine_id.0.as_str(),
                process_id.0.as_str(),
                class_type.as_str(),
                &process.address.to_string(),
            ];
            process.to_metrics(&labels);
        }

        for (role, count) in self.cluster_roles_count() {
            P_CLUSTER_PROCESS_ROLES_COUNT
                .with_label_values(&[&role.to_string()])
                .set(count as i64);
        }

        if let Some(latency_probe) = &self.latency_probe {
            latency_probe.to_metrics(&[]);
        }

        if let Some(layers) = &self.layers {
            if let Some(backup) = &layers.backup {
                backup.to_metrics(&[]);
            }
        }

        if let Some(wiggle) = &self.storage_wiggler {
            wiggle.to_metrics(&[]);
        }

        P_CLUSTER_GENERATION_COUNT.set(self.generation);

        if let Some(qos) = &self.qos {
            qos.to_metrics(&[]);
        }
    }
}
