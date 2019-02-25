//! TODO

#![stable(feature = "rust1", since = "1.0.0")]

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::ip::{IpAddr, Ipv4Addr, Ipv6Addr, Ipv6MulticastScope};

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::parser::AddrParseError;

mod ip;
mod parser;
