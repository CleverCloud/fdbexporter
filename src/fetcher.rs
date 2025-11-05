use std::path::Path;

use foundationdb::{options::TransactionOption, Database, FdbBindingError, FdbError};
use tracing::error;

use crate::status_models::Status;

/// Errors that can occur when fetching cluster status
#[derive(Debug)]
pub enum FetchError {
    /// Error parsing JSON status output
    Parsing(serde_path_to_error::Error<serde_json::Error>),
    /// Error from FoundationDB operations
    Fdb(FdbError),
    /// Error from FoundationDB binding operations
    FdbBinding(FdbBindingError),
    /// Error when the status key is not found
    StatusNotFound,
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchError::Parsing(e) => write!(f, "Failed to parse status JSON: {}", e),
            FetchError::Fdb(e) => write!(f, "FoundationDB error: {}", e),
            FetchError::FdbBinding(e) => write!(f, "FoundationDB binding error: {}", e),
            FetchError::StatusNotFound => write!(f, "Status key not found in FoundationDB"),
        }
    }
}

impl std::error::Error for FetchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FetchError::Parsing(e) => Some(e),
            FetchError::Fdb(e) => Some(e),
            FetchError::FdbBinding(e) => Some(e),
            FetchError::StatusNotFound => None,
        }
    }
}

impl From<FdbError> for FetchError {
    fn from(e: FdbError) -> Self {
        FetchError::Fdb(e)
    }
}

impl From<FdbBindingError> for FetchError {
    fn from(e: FdbBindingError) -> Self {
        FetchError::FdbBinding(e)
    }
}

/// Fetches the FoundationDB cluster status by reading the system key `\xff\xff/status/json`.
///
/// # Arguments
///
/// * `cluster_file` - Optional path to the cluster file. If None, uses the default cluster file.
///
/// # Returns
///
/// Returns `Ok(Status)` if the status key can be read and parsed successfully,
/// otherwise returns a `FetchError`.
///
/// # Examples
///
/// ```no_run
/// use fdbexporter::fetch_cluster_status;
/// use std::path::Path;
///
/// # async fn example() -> Result<(), fdbexporter::FetchError> {
/// // Use default cluster file
/// let status = fetch_cluster_status(None).await?;
///
/// // Use custom cluster file
/// let status = fetch_cluster_status(Some(Path::new("/etc/foundationdb/fdb.cluster"))).await?;
/// # Ok(())
/// # }
/// ```
pub async fn fetch_cluster_status(cluster_file: Option<&Path>) -> Result<Status, FetchError> {
    let db = if let Some(path) = cluster_file {
        let path_str = path.to_str().ok_or_else(|| {
            // Create a custom error for invalid path
            FetchError::FdbBinding(FdbBindingError::CustomError(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid cluster file path",
            ))))
        })?;
        Database::from_path(path_str)?
    } else {
        Database::default()?
    };

    // Read the status JSON from the system key
    let status_json = db
        .run(|trx, _maybe_committed| async move {
            // Set the option to read system keys
            trx.set_option(TransactionOption::ReadSystemKeys)?;

            // The status JSON is stored at the special key \xff\xff/status/json
            let status_key = b"\xff\xff/status/json";

            // Read the key
            let value = trx.get(status_key, false).await?;

            Ok(value)
        })
        .await?;

    // Check if the key exists
    let json_bytes = status_json.ok_or(FetchError::StatusNotFound)?;

    // Parse the JSON
    let json_status = &mut serde_json::Deserializer::from_slice(&json_bytes);
    serde_path_to_error::deserialize(json_status).map_err(|e| {
        error!("Couldn't parse json: {}", e);
        FetchError::Parsing(e)
    })
}
