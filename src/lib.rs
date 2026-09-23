//! # fdbexporter
//!
//! A library for fetching and processing FoundationDB cluster status with support
//! for exporting metrics in Prometheus format.
//!
//! ## Core Functionality
//!
//! This library provides:
//! - Parsing the JSON status document (`status json`) into strongly-typed Rust
//!   structures ([`status_models`], [`parse_status`])
//! - Converting status data into Prometheus metrics ([`process_metrics`])
//! - With the `fdb` feature, direct access to FoundationDB cluster status via the
//!   system key `\xff\xff/status/json` (`fetch_cluster_status`)
//!
//! ## Usage modes
//!
//! - **With FoundationDB** (default): the `fdb` feature links the native FoundationDB
//!   client (fdb-rs), `fetch_cluster_status` reads the status from a cluster and
//!   [`process_metrics`] exports it.
//! - **Without FoundationDB** (`default-features = false`): only the status models,
//!   [`parse_status`] and the Prometheus metrics are built, with no `foundationdb`
//!   crate in the dependency graph. Use it when the application already links a
//!   FoundationDB client, or gets the status JSON another way, and must not carry a
//!   second copy of fdb-rs.
//!
//! ```toml
//! [dependencies]
//! fdbexporter = { git = "https://github.com/CleverCloud/fdbexporter.git", default-features = false }
//! ```
//!
//! ## Features
//!
//! - `fdb`: the native FoundationDB client and `fetch_cluster_status`. It needs one
//!   version feature: fdb-rs rejects a build with `fdb` alone.
//! - `fdb-7_1`, `fdb-7_3`: the FoundationDB API version. Each implies `fdb`, and they
//!   are mutually exclusive.
//! - `binary`: the `fdbexporter` executable (HTTP server and CLI). Implies `fdb`.
//!
//! The default features are `binary`, `fdb` and `fdb-7_3`.
//!
//! ## Important Notes
//!
//! With the `fdb` feature, the FoundationDB client must be initialized once per
//! process using `unsafe { foundationdb::boot() }` before calling
//! `fetch_cluster_status`, and the returned guard dropped before the process exits.
//! `fetch_cluster_status` is async and needs an async runtime; its documentation has
//! an example. Without `fdb` there is nothing to initialize and the API is synchronous.
//!
//! ## Example
//!
//! Export a status document obtained without the `fdb` feature:
//!
//! ```
//! use fdbexporter::{parse_status, process_metrics, MetricsConvertible};
//! use prometheus::{Encoder, TextEncoder};
//!
//! fn export(json: &[u8]) {
//!     match parse_status(json) {
//!         Ok(status) => process_metrics(status),
//!         // Failures are exported too, here as `fdb_exporter_parsing_error_count`.
//!         Err(e) => e.to_metrics(&[]),
//!     }
//! }
//!
//! // A `status json` document trimmed to its mandatory fields.
//! export(br#"{
//!     "client": {
//!         "coordinators": {
//!             "coordinators": [{ "address": "10.0.0.1:4500", "reachable": true }],
//!             "quorum_reachable": true
//!         },
//!         "database_status": { "available": true, "healthy": true },
//!         "messages": []
//!     },
//!     "cluster": { "generation": 2 }
//! }"#);
//! export(b"not a status document");
//!
//! let mut buffer = Vec::new();
//! TextEncoder::new().encode(&prometheus::gather(), &mut buffer)?;
//! let text = String::from_utf8(buffer)?;
//! assert!(text.contains("fdb_database_available 1"));
//! assert!(text.contains("fdb_cluster_generation_count 2"));
//! assert!(text.contains("fdb_exporter_parsing_error_count"));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

// Public module declarations
pub mod fetcher;
pub mod metrics;
pub mod status_models;

// Re-export commonly used types and functions
#[cfg(feature = "fdb")]
pub use fetcher::fetch_cluster_status;
pub use fetcher::{parse_status, FetchError};
pub use metrics::{process_metrics, MetricsConvertible};
pub use status_models::Status;
