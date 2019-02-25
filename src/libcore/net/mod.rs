//! Networking primitives.
//!
//! This module provides types for IP and socket addresses.
//!
//! # Organization
//!
//! * [`IpAddr`] represents IP addresses of either IPv4 or IPv6; [`Ipv4Addr`] and
//!   [`Ipv6Addr`] are respectively IPv4 and IPv6 addresses
//! * [`SocketAddr`] represents socket addresses of either IPv4 or IPv6; [`SocketAddrV4`]
//!   and [`SocketAddrV6`] are respectively IPv4 and IPv6 socket addresses
//! * Other types are return or parameter types for various methods in this module
//!
//! [`IpAddr`]: ../../core/net/enum.IpAddr.html
//! [`Ipv4Addr`]: ../../core/net/struct.Ipv4Addr.html
//! [`Ipv6Addr`]: ../../core/net/struct.Ipv6Addr.html
//! [`SocketAddr`]: ../../core/net/enum.SocketAddr.html
//! [`SocketAddrV4`]: ../../core/net/struct.SocketAddrV4.html
//! [`SocketAddrV6`]: ../../core/net/struct.SocketAddrV6.html


#![stable(feature = "rust1", since = "1.0.0")]

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::ip::{IpAddr, Ipv4Addr, Ipv6Addr, Ipv6MulticastScope};

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::addr::{SocketAddr, SocketAddrV4, SocketAddrV6};

#[stable(feature = "rust1", since = "1.0.0")]
pub use self::parser::AddrParseError;

mod ip;
mod addr;
mod parser;
