use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Eq, PartialEq, PartialOrd, Hash)]
pub struct BackupId(pub String);

#[derive(Deserialize)]
pub struct ClusterBackup {
    pub paused: bool,
    pub total_workers: Option<i64>,
    pub instances_running: Option<i64>,
    pub blob_recent_io: Option<ClusterBackupRecentIo>,

    pub tags: HashMap<BackupId, ClusterBackupTag>,
}

#[derive(Deserialize)]
pub struct ClusterBackupTag {
    pub last_restorable_seconds_behind: Option<f64>,
    pub last_restorable_version: Option<i64>,
    pub running_backup: bool,
    pub running_backup_is_restorable: bool,
    pub range_bytes_written: i64,
    pub mutation_log_bytes_written: i64,
}

#[derive(Deserialize)]
pub struct ClusterBackupRecentIo {
    pub bytes_per_second: f64,
    pub bytes_sent: i64,
    pub requests_failed: i64,
    pub requests_successful: i64,
}
