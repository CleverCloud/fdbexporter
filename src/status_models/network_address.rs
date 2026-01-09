use std::fmt;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};
use std::str::FromStr;

use serde::de::{self, Visitor};
use serde::Deserialize;

/// A network address that can be an IPv4/IPv6 socket address or a DNS hostname with port.
/// This is needed because FoundationDB Kubernetes deployments use DNS-based cluster files
/// when `useDNSInClusterFile: true` is set (default for Kubernetes deployments).
///
/// Supports the following formats:
/// - IPv4: "10.0.0.1:4500"
/// - IPv6: "[::1]:4500" or "[2001:db8::1]:4500"
/// - DNS:  "hostname.namespace.svc.cluster.local:4501"
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkAddress {
    /// IPv4 socket address (e.g., "10.0.0.1:4500")
    Ipv4(SocketAddrV4),
    /// IPv6 socket address (e.g., "[::1]:4500")
    Ipv6(SocketAddrV6),
    /// DNS hostname with port (e.g., "hostname.namespace.svc.cluster.local:4501")
    Dns { hostname: String, port: u16 },
}

impl fmt::Display for NetworkAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkAddress::Ipv4(addr) => write!(f, "{}", addr),
            NetworkAddress::Ipv6(addr) => write!(f, "{}", addr),
            NetworkAddress::Dns { hostname, port } => write!(f, "{}:{}", hostname, port),
        }
    }
}

impl FromStr for NetworkAddress {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // First, try to parse as a standard socket address (IPv4 or IPv6)
        if let Ok(addr) = s.parse::<SocketAddr>() {
            return match addr {
                SocketAddr::V4(v4) => Ok(NetworkAddress::Ipv4(v4)),
                SocketAddr::V6(v6) => Ok(NetworkAddress::Ipv6(v6)),
            };
        }

        // Otherwise, try to parse as DNS hostname:port
        // For IPv6 in brackets like [::1]:4500, the above parse would have succeeded,
        // so here we only handle hostname:port format

        // Find the last colon to split hostname and port
        if let Some(colon_pos) = s.rfind(':') {
            let hostname = &s[..colon_pos];
            let port_str = &s[colon_pos + 1..];

            // Validate hostname is not empty
            if hostname.is_empty() {
                return Err(format!("Invalid network address: empty hostname in '{}'", s));
            }

            // Parse port
            let port = port_str
                .parse::<u16>()
                .map_err(|_| format!("Invalid port number in network address: '{}'", s))?;

            Ok(NetworkAddress::Dns {
                hostname: hostname.to_string(),
                port,
            })
        } else {
            Err(format!(
                "Invalid network address format: '{}', expected 'host:port'",
                s
            ))
        }
    }
}

struct NetworkAddressVisitor;

impl<'de> Visitor<'de> for NetworkAddressVisitor {
    type Value = NetworkAddress;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a network address in the form 'ip:port', '[ipv6]:port', or 'hostname:port'")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NetworkAddress::from_str(value).map_err(de::Error::custom)
    }
}

impl<'de> Deserialize<'de> for NetworkAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(NetworkAddressVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn test_parse_ipv4_address() {
        let addr: NetworkAddress = "10.0.0.1:4500".parse().unwrap();
        match addr {
            NetworkAddress::Ipv4(socket_addr) => {
                assert_eq!(socket_addr.ip(), &Ipv4Addr::new(10, 0, 0, 1));
                assert_eq!(socket_addr.port(), 4500);
            }
            _ => panic!("Expected IPv4 address"),
        }
    }

    #[test]
    fn test_parse_ipv6_address() {
        let addr: NetworkAddress = "[::1]:4500".parse().unwrap();
        match addr {
            NetworkAddress::Ipv6(socket_addr) => {
                assert_eq!(socket_addr.ip(), &Ipv6Addr::LOCALHOST);
                assert_eq!(socket_addr.port(), 4500);
            }
            _ => panic!("Expected IPv6 address"),
        }
    }

    #[test]
    fn test_parse_ipv6_full_address() {
        let addr: NetworkAddress = "[2001:db8::1]:4501".parse().unwrap();
        match addr {
            NetworkAddress::Ipv6(socket_addr) => {
                assert_eq!(
                    socket_addr.ip(),
                    &Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1)
                );
                assert_eq!(socket_addr.port(), 4501);
            }
            _ => panic!("Expected IPv6 address"),
        }
    }

    #[test]
    fn test_parse_dns_address() {
        let addr: NetworkAddress =
            "foundationdb-log-39303.foundationdb.doris.svc.cluster.local:4501"
                .parse()
                .unwrap();
        match addr {
            NetworkAddress::Dns { hostname, port } => {
                assert_eq!(
                    hostname,
                    "foundationdb-log-39303.foundationdb.doris.svc.cluster.local"
                );
                assert_eq!(port, 4501);
            }
            _ => panic!("Expected DNS address"),
        }
    }

    #[test]
    fn test_parse_simple_hostname() {
        let addr: NetworkAddress = "localhost:4500".parse().unwrap();
        match addr {
            NetworkAddress::Dns { hostname, port } => {
                assert_eq!(hostname, "localhost");
                assert_eq!(port, 4500);
            }
            _ => panic!("Expected DNS address"),
        }
    }

    #[test]
    fn test_display_ipv4() {
        let addr = NetworkAddress::Ipv4(SocketAddrV4::new(Ipv4Addr::new(192, 168, 1, 1), 4500));
        assert_eq!(addr.to_string(), "192.168.1.1:4500");
    }

    #[test]
    fn test_display_ipv6() {
        let addr = NetworkAddress::Ipv6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 4500, 0, 0));
        assert_eq!(addr.to_string(), "[::1]:4500");
    }

    #[test]
    fn test_display_dns() {
        let addr = NetworkAddress::Dns {
            hostname: "fdb.example.com".to_string(),
            port: 4501,
        };
        assert_eq!(addr.to_string(), "fdb.example.com:4501");
    }

    #[test]
    fn test_invalid_address_no_port() {
        let result: Result<NetworkAddress, _> = "hostname".parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_address_empty_hostname() {
        let result: Result<NetworkAddress, _> = ":4500".parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_port() {
        let result: Result<NetworkAddress, _> = "hostname:invalid".parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_ipv4() {
        let json = "\"10.0.0.1:4500\"";
        let addr: NetworkAddress = serde_json::from_str(json).unwrap();
        match addr {
            NetworkAddress::Ipv4(socket_addr) => {
                assert_eq!(socket_addr.ip(), &Ipv4Addr::new(10, 0, 0, 1));
                assert_eq!(socket_addr.port(), 4500);
            }
            _ => panic!("Expected IPv4 address"),
        }
    }

    #[test]
    fn test_deserialize_ipv6() {
        let json = "\"[::1]:4500\"";
        let addr: NetworkAddress = serde_json::from_str(json).unwrap();
        match addr {
            NetworkAddress::Ipv6(socket_addr) => {
                assert_eq!(socket_addr.ip(), &Ipv6Addr::LOCALHOST);
                assert_eq!(socket_addr.port(), 4500);
            }
            _ => panic!("Expected IPv6 address"),
        }
    }

    #[test]
    fn test_deserialize_dns() {
        let json = "\"fdb-coordinator.default.svc.cluster.local:4501\"";
        let addr: NetworkAddress = serde_json::from_str(json).unwrap();
        match addr {
            NetworkAddress::Dns { hostname, port } => {
                assert_eq!(hostname, "fdb-coordinator.default.svc.cluster.local");
                assert_eq!(port, 4501);
            }
            _ => panic!("Expected DNS address"),
        }
    }
}
