use serde::Deserialize;

use crate::status_models::address::FdbProcessAddress;

/// jq: .client
#[derive(Deserialize)]
pub struct ClientStatus {
    pub coordinators: ClientCoordinators,
    pub timestamp: Option<i64>,
    pub database_status: ClientDatabaseStatus,
    pub messages: Vec<ClientMessage>,
}

/// jq: .client.messages[]
#[derive(Deserialize)]
pub struct ClientMessage {
    /// Can only be a discrete list of values:
    /// - inconsistent_cluster_file
    /// - no_cluster_controller
    /// - quorum_not_reachable
    /// - server_overloaded
    /// - status_cimplete-client
    /// - status_incompelte_cluster
    /// - status_incomplete_coordinators
    /// - status_incomplete_error
    /// - status_incomplete_timeout
    /// - unreachable_cluster_controller
    pub name: String,
    pub description: String,
}

/// jq: .client.database_status
#[derive(Deserialize)]
pub struct ClientDatabaseStatus {
    pub available: bool,
    pub healthy: bool,
}

/// jq: .client.coordinators
#[derive(Deserialize)]
pub struct ClientCoordinators {
    pub coordinators: Vec<ClientCoordinator>,
    pub quorum_reachable: bool,
}

/// jq: .client.coordinators.coordinator
#[derive(Deserialize)]
pub struct ClientCoordinator {
    pub address: FdbProcessAddress,
    pub protocol: Option<String>,
    pub reachable: bool,
}
