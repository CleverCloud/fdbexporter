use serde::Deserialize;

use std::fmt;
use std::net::SocketAddrV4;
use std::net::SocketAddrV6;
use either::Either;

pub struct Address {
    pub address: Either<SocketAddrV4, SocketAddrV6>,
    /// can only be the literal string ":tls" or absent
    pub tls: bool,
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let tls = s.ends_with(":tls");
        let addr_str = if tls { s.trim_end_matches(":tls") } else { &s };

        let address = if addr_str.contains("::") {
            Either::Right(addr_str.parse().map_err(serde::de::Error::custom)?)
        } else {
            Either::Left(addr_str.parse().map_err(serde::de::Error::custom)?)
        };

        Ok(Address { address, tls })
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.tls {
            write!(f, "{}:tls", self.address)
        } else {
            write!(f, "{}", self.address)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;
    
    #[test]
    fn ipv4_without_tls() {
        let addr = "127.0.0.1:1234";
        let json = serde_json::json!(addr);
        let deserialized: Address = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.address.to_string(), "127.0.0.1:1234");
        assert!(!deserialized.tls);

        let round_trip = deserialized.to_string();
        assert_eq!(round_trip, addr)
    }
    
    #[test]
    fn ipv4_with_tls() {
        let addr = "127.0.0.1:1234:tls";
        let json = serde_json::json!(addr);
        let deserialized: Address = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.address.to_string(), "127.0.0.1:1234");
        assert!(deserialized.tls);

        let round_trip = deserialized.to_string();
        assert_eq!(round_trip, addr)
    }
    
    #[test]
    fn ipv6_without_tls() {
        let addr = "[::1]:4500";
        let json = serde_json::json!(addr);
        let deserialized: Address = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.address.to_string(), "[::1]:4500");
        assert!(!deserialized.tls);

        let round_trip = deserialized.to_string();
        assert_eq!(round_trip, addr)
    }
    
    #[test]
    fn ipv6_with_tls() {
        let addr = "[::1]:4500:tls";
        let json = serde_json::json!(addr);
        let deserialized: Address = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.address.to_string(), "[::1]:4500");
        assert!(deserialized.tls);

        let round_trip = deserialized.to_string();
        assert_eq!(round_trip, addr)
    }
}
