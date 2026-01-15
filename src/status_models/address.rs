use serde::{de, Deserialize};

use std::{fmt, u16};
use url::Host;

#[derive(Debug)]
pub enum AddressError {
    MissingPort,
    ParsingPort,
    ParsingHost,
}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AddressError::MissingPort => write!(f, "Missing port"),
            AddressError::ParsingPort => write!(f, "Parsing port"),
            AddressError::ParsingHost => write!(f, "Parsing host"),
        }
    }
}

pub struct FdbProcessAddress {
    pub host: Host<String>,
    pub port: u16,
    pub tls: bool,
}

const TLS_SUFFIX: &str = ":tls";

impl FdbProcessAddress {
    pub fn new(host: Host<String>, port: u16, tls: bool) -> Self {
        Self { host, port, tls }
    }

    pub fn parse(s: &str) -> Result<Self, AddressError> {
        let tls = s.ends_with(TLS_SUFFIX);
        let host_port = if tls {
            &s[..s.len() - TLS_SUFFIX.len()]
        } else {
            s
        };

        let port_pos = host_port.rfind(":").ok_or(AddressError::MissingPort)?;

        let port_str = &host_port[port_pos + 1..];
        let port = port_str
            .parse::<u16>()
            .map_err(|_| AddressError::ParsingPort)?;

        let host_str = &host_port[..port_pos];
        let host = url::Host::parse(host_str).map_err(|_| AddressError::ParsingHost)?;

        return Ok(FdbProcessAddress::new(host, port, tls));
    }
}

impl<'de> Deserialize<'de> for FdbProcessAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FdbProcessAddress::parse(s.as_str()).map_err(|e| de::Error::custom(e))
    }
}

impl fmt::Display for FdbProcessAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)?;
        if self.tls {
            write!(f, ":tls")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::net::{Ipv4Addr, Ipv6Addr};
    use url::Host;

    use super::*;

    #[test]
    fn ipv4_without_tls() {
        let addr = "127.0.0.1:1234";
        let deserialized = FdbProcessAddress::parse(addr).unwrap();
        assert_eq!(
            deserialized.host,
            Host::<String>::Ipv4(Ipv4Addr::new(127, 0, 0, 1))
        );
        assert_eq!(deserialized.port, 1234u16);
        assert!(!deserialized.tls);

        let round_trip = deserialized.to_string();
        assert_eq!(round_trip, addr)
    }

    #[test]
    fn ipv4_with_tls() {
        let addr = "127.0.0.1:1234:tls";
        let deserialized = FdbProcessAddress::parse(addr).unwrap();
        assert_eq!(
            deserialized.host,
            Host::<String>::Ipv4(Ipv4Addr::new(127, 0, 0, 1))
        );
        assert_eq!(deserialized.port, 1234u16);
        assert!(deserialized.tls);

        let round_trip = deserialized.to_string();
        assert_eq!(round_trip, addr)
    }

    #[test]
    fn ipv6_without_tls() {
        let addr = "[::1]:4500";
        let deserialized = FdbProcessAddress::parse(addr).unwrap();
        assert_eq!(
            deserialized.host,
            Host::<String>::Ipv6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))
        );
        assert_eq!(deserialized.port, 4500u16);
        assert!(!deserialized.tls);

        let round_trip = deserialized.to_string();
        assert_eq!(round_trip, addr)
    }

    #[test]
    fn ipv6_with_tls() {
        let addr = "[::1]:4500:tls";
        let deserialized = FdbProcessAddress::parse(addr).unwrap();
        assert_eq!(
            deserialized.host,
            Host::<String>::Ipv6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))
        );
        assert_eq!(deserialized.port, 4500u16);
        assert!(deserialized.tls);

        let round_trip = deserialized.to_string();
        assert_eq!(round_trip, addr)
    }

    #[test]
    fn dns_without_tls() {
        let addr = "somedomain.com:4500";
        let deserialized = FdbProcessAddress::parse(addr).unwrap();
        assert_eq!(
            deserialized.host,
            Host::<String>::Domain("somedomain.com".to_string())
        );
        assert_eq!(deserialized.port, 4500u16);
        assert!(!deserialized.tls);

        let round_trip = deserialized.to_string();
        assert_eq!(round_trip, addr)
    }

    #[test]
    fn dns_with_tls() {
        let addr = "somedomain.com:4500:tls";
        let deserialized = FdbProcessAddress::parse(addr).unwrap();
        assert_eq!(
            deserialized.host,
            Host::<String>::Domain("somedomain.com".to_string())
        );
        assert_eq!(deserialized.port, 4500u16);
        assert!(deserialized.tls);

        let round_trip = deserialized.to_string();
        assert_eq!(round_trip, addr)
    }
}
