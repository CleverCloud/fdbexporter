use bytes::Bytes;
use clap::Parser;
use fdbexporter::{fetch_cluster_status, process_metrics, FetchError, MetricsConvertible};
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use prometheus::{Encoder, TextEncoder};

use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::num::ParseIntError;
use std::path::PathBuf;

use tokio::{
    net::TcpListener,
    time::{sleep, Duration},
};
use tracing::{error, info};

async fn metrics(_: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    Ok(Response::new(Full::new(buffer.into())))
}

async fn run_http_server(config: &CommandArgs) -> Result<(), anyhow::Error> {
    let addr: SocketAddr = (config.addr, config.port).into();
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on http://{}", addr);
    loop {
        let (tcp, _) = listener.accept().await?;
        let io = TokioIo::new(tcp);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(metrics))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}

/// Run a loop which will fetch regularly FDB status from the system key, to fetch current state
/// of the cluster.
async fn run_status_fetcher(config: &CommandArgs) -> Result<(), anyhow::Error> {
    let cluster_path = config.cluster.as_deref();

    loop {
        let status = fetch_cluster_status(cluster_path).await;

        match status {
            Ok(status) => process_metrics(status),
            Err(FetchError::FdbBinding(e)) => {
                return Err(e.into());
            }
            Err(e) => e.to_metrics(&[]),
        };
        sleep(config.delay_sec).await;
    }
}

/// FoundationDB exporter for metrics parsed from status
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CommandArgs {
    /// Listening port of the web server
    #[arg(short, long, default_value_t = 9090, env = "FDB_EXPORTER_PORT")]
    port: u16,

    /// Listen address of the web server, can be IPv4 or IPv6
    #[arg(short, long, default_value = "0.0.0.0", env = "FDB_EXPORTER_ADDR")]
    addr: IpAddr,

    /// Location of fdb.cluster file
    #[arg(short, long, env = "FDB_CLUSTER_FILE")]
    cluster: Option<PathBuf>,

    /// Delay in seconds between two update of the status & metrics
    #[arg(short, long, env = "FDB_EXPORTER_DELAY", value_parser = parse_duration, default_value = "15")]
    delay_sec: Duration,
}

fn parse_duration(arg: &str) -> Result<Duration, ParseIntError> {
    let seconds = arg.parse()?;
    Ok(Duration::from_secs(seconds))
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();

    // Initialize FoundationDB client
    // Safe because we drop it before the program exits
    let _fdb_network = unsafe { foundationdb::boot() };

    let cli = CommandArgs::parse();

    tokio::select! {
        server = run_http_server(&cli) => {
            if let Err(err) = server {
                error!("HTTP server thread failed, {:?}", err);
            }
        },
        fetcher = run_status_fetcher(&cli) => {
            if let Err(err) = fetcher {
                error!("HTTP fetcher thread failed, {:?}", err);
            }
        },
    };

    // Clean shutdown of FDB network
    drop(_fdb_network);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{net::Ipv4Addr, time::Duration};

    use crate::CommandArgs;

    impl Default for CommandArgs {
        fn default() -> Self {
            CommandArgs {
                port: 9090,
                addr: std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                cluster: None,
                delay_sec: Duration::from_secs(1),
            }
        }
    }
}
