//! Fetching and parsing of the FoundationDB `status json` document.
//!
//! [`parse_status`] is always available. `fetch_cluster_status`, which reads the
//! document from a cluster with the native client, requires the `fdb` feature.

#[cfg(feature = "fdb")]
use std::{path::Path, time::Duration};

#[cfg(feature = "fdb")]
use foundationdb::{options::TransactionOption, Database, FdbBindingError, FdbError};
use tracing::error;

use crate::status_models::Status;

/// Errors that can occur when fetching or parsing cluster status
///
/// The `Fdb` and `FdbBinding` variants only exist with the `fdb` feature. Cargo
/// unifies features across a build, so they appear as soon as any crate in the build
/// enables `fdb`: code written without `fdb` should not match on this enum exhaustively.
#[derive(Debug)]
pub enum FetchError {
    /// Error parsing JSON status output
    Parsing(serde_path_to_error::Error<serde_json::Error>),
    /// Error from FoundationDB operations
    #[cfg(feature = "fdb")]
    Fdb(FdbError),
    /// Error from FoundationDB binding operations
    #[cfg(feature = "fdb")]
    FdbBinding(FdbBindingError),
    /// Error when the status key is not found
    StatusNotFound,
    /// Error when the requested timeout is too large
    TimeoutTooLarge(u128),
}

impl std::fmt::Display for FetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FetchError::Parsing(e) => write!(f, "Failed to parse status JSON: {}", e),
            #[cfg(feature = "fdb")]
            FetchError::Fdb(e) => write!(f, "FoundationDB error: {}", e),
            #[cfg(feature = "fdb")]
            FetchError::FdbBinding(e) => write!(f, "FoundationDB binding error: {}", e),
            FetchError::StatusNotFound => write!(f, "Status key not found in FoundationDB"),
            FetchError::TimeoutTooLarge(ms) => {
                write!(
                    f,
                    "Timeout value {}ms exceeds maximum of {}ms (i32::MAX)",
                    ms,
                    i32::MAX
                )
            }
        }
    }
}

impl std::error::Error for FetchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FetchError::Parsing(e) => Some(e),
            #[cfg(feature = "fdb")]
            FetchError::Fdb(e) => Some(e),
            #[cfg(feature = "fdb")]
            FetchError::FdbBinding(e) => Some(e),
            FetchError::StatusNotFound => None,
            FetchError::TimeoutTooLarge(_) => None,
        }
    }
}

#[cfg(feature = "fdb")]
impl From<FdbError> for FetchError {
    fn from(e: FdbError) -> Self {
        FetchError::Fdb(e)
    }
}

#[cfg(feature = "fdb")]
impl From<FdbBindingError> for FetchError {
    fn from(e: FdbBindingError) -> Self {
        FetchError::FdbBinding(e)
    }
}

/// Parses a FoundationDB `status json` document into a [`Status`].
///
/// This is the parser behind `fetch_cluster_status`. It is available without the
/// `fdb` feature, for applications that read the `\xff\xff/status/json` key with
/// their own FoundationDB client or get the document from `fdbcli`.
///
/// # Errors
///
/// Returns [`FetchError::Parsing`] when `json` is not valid JSON or does not match
/// the [`Status`] model. The inner error carries the path of the offending field.
/// The failure is also logged with `tracing::error!`.
///
/// # Examples
///
/// ```
/// use fdbexporter::{parse_status, FetchError};
///
/// let json = br#"{
///     "client": {
///         "coordinators": { "coordinators": [], "quorum_reachable": false },
///         "database_status": { "available": false, "healthy": false },
///         "messages": []
///     }
/// }"#;
/// let status = parse_status(json)?;
/// assert!(!status.client.database_status.available);
/// assert!(status.cluster.is_none());
///
/// // The error tells which field does not match the model.
/// match parse_status(br#"{ "client": { "coordinators": 42 } }"#) {
///     Err(FetchError::Parsing(e)) => assert_eq!(e.path().to_string(), "client.coordinators"),
///     _ => panic!("expected a parsing error"),
/// }
/// # Ok::<(), FetchError>(())
/// ```
pub fn parse_status(json: &[u8]) -> Result<Status, FetchError> {
    let json_status = &mut serde_json::Deserializer::from_slice(json);
    serde_path_to_error::deserialize(json_status).map_err(|e| {
        error!("Couldn't parse json: {}", e);
        FetchError::Parsing(e)
    })
}

/// Fetches the FoundationDB cluster status by reading the system key `\xff\xff/status/json`.
///
/// Requires the `fdb` feature. The FoundationDB client must be initialized once per
/// process with `unsafe { foundationdb::boot() }`, and the returned guard dropped
/// before the process exits. The document is parsed with [`parse_status`].
///
/// # Arguments
///
/// * `cluster_file` - Optional path to the cluster file. If None, uses the default cluster file.
/// * `timeout_duration` - Timeout to use when querying for status.
///
/// # Returns
///
/// Returns `Ok(Status)` if the status key can be read and parsed successfully,
/// otherwise returns a `FetchError`.
///
/// # Examples
///
/// ```no_run
/// use fdbexporter::{fetch_cluster_status, process_metrics, MetricsConvertible};
/// use std::{path::Path, time::Duration};
///
/// # async fn example() -> Result<(), fdbexporter::FetchError> {
/// // Initialize the FoundationDB client (once per process)
/// let network = unsafe { foundationdb::boot() };
///
/// let timeout = Duration::new(15, 0);
///
/// // Use default cluster file, exporting failures as `fdb_exporter_*` counters
/// match fetch_cluster_status(None, timeout).await {
///     Ok(status) => process_metrics(status),
///     Err(e) => e.to_metrics(&[]),
/// }
///
/// // Use custom cluster file
/// let status = fetch_cluster_status(Some(Path::new("/etc/foundationdb/fdb.cluster")), timeout).await?;
/// process_metrics(status);
///
/// // Stop the FoundationDB network before exiting
/// drop(network);
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "fdb")]
pub async fn fetch_cluster_status(
    cluster_file: Option<&Path>,
    timeout_duration: Duration,
) -> Result<Status, FetchError> {
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

    let timeout_millis = timeout_duration
        .as_millis()
        .try_into()
        .map_err(|_| FetchError::TimeoutTooLarge(timeout_duration.as_millis()))?;

    // Read the status JSON from the system key
    let status_json = db
        .run(|trx, _maybe_committed| async move {
            // Set the option to read system keys
            trx.set_option(TransactionOption::ReadSystemKeys)?;
            trx.set_option(TransactionOption::Timeout(timeout_millis))?;

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
    parse_status(&json_bytes)
}
