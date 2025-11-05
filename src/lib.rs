//! # fdbexporter
//!
//! A library for fetching and processing FoundationDB cluster status with support
//! for exporting metrics in Prometheus format.
//!
//! ## Core Functionality
//!
//! This library provides:
//! - Direct access to FoundationDB cluster status via the system key `\xff\xff/status/json`
//! - Parsing JSON status output into strongly-typed Rust structures
//! - Converting status data into Prometheus metrics
//!
//! ## Important Notes
//!
//! This library now requires async runtime support. The FoundationDB client must be initialized
//! once per process using `unsafe { foundationdb::boot() }` before using any functions.
//!
//! ## Example
//!
//! ```no_run
//! use fdbexporter::{fetch_cluster_status, process_metrics};
//! use std::path::Path;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Initialize FoundationDB client (once per process)
//! let _guard = unsafe { foundationdb::boot() };
//!
//! // Fetch status using default cluster file
//! match fetch_cluster_status(None).await {
//!     Ok(status) => process_metrics(status),
//!     Err(e) => eprintln!("Failed to fetch status: {:?}", e),
//! }
//!
//! // Or use a custom cluster file
//! match fetch_cluster_status(Some(Path::new("/etc/foundationdb/fdb.cluster"))).await {
//!     Ok(status) => process_metrics(status),
//!     Err(e) => eprintln!("Failed to fetch status: {:?}", e),
//! }
//!
//! // Clean shutdown
//! drop(_guard);
//! # Ok(())
//! # }
//! ```

// Public module declarations
pub mod fetcher;
pub mod metrics;
pub mod status_models;

// Re-export commonly used types and functions
pub use fetcher::{fetch_cluster_status, FetchError};
pub use metrics::{process_metrics, MetricsConvertible};
pub use status_models::Status;
