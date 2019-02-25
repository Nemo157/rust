use fmt;
use net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// An internet socket address, either IPv4 or IPv6.
///
/// Internet socket addresses consist of an [IP address], a 16-bit port number, as well
/// as possibly some version-dependent additional information. See [`SocketAddrV4`]'s and
/// [`SocketAddrV6`]'s respective documentation for more details.
///
/// The size of a `SocketAddr` instance may vary depending on the target operating
/// system.
///
/// [IP address]: ../../std/net/enum.IpAddr.html
/// [`SocketAddrV4`]: ../../std/net/struct.SocketAddrV4.html
/// [`SocketAddrV6`]: ../../std/net/struct.SocketAddrV6.html
///
/// # Examples
///
/// ```
/// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
///
/// let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
///
/// assert_eq!("127.0.0.1:8080".parse(), Ok(socket));
/// assert_eq!(socket.port(), 8080);
/// assert_eq!(socket.is_ipv4(), true);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[stable(feature = "rust1", since = "1.0.0")]
pub enum SocketAddr {
    /// An IPv4 socket address.
    #[stable(feature = "rust1", since = "1.0.0")]
    V4(#[stable(feature = "rust1", since = "1.0.0")] SocketAddrV4),
    /// An IPv6 socket address.
    #[stable(feature = "rust1", since = "1.0.0")]
    V6(#[stable(feature = "rust1", since = "1.0.0")] SocketAddrV6),
}

/// An IPv4 socket address.
///
/// IPv4 socket addresses consist of an [IPv4 address] and a 16-bit port number, as
/// stated in [IETF RFC 793].
///
/// See [`SocketAddr`] for a type encompassing both IPv4 and IPv6 socket addresses.
///
/// The size of a `SocketAddrV4` struct may vary depending on the target operating
/// system.
///
/// [IETF RFC 793]: https://tools.ietf.org/html/rfc793
/// [IPv4 address]: ../../std/net/struct.Ipv4Addr.html
/// [`SocketAddr`]: ../../std/net/enum.SocketAddr.html
///
/// # Examples
///
/// ```
/// use std::net::{Ipv4Addr, SocketAddrV4};
///
/// let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
///
/// assert_eq!("127.0.0.1:8080".parse(), Ok(socket));
/// assert_eq!(socket.ip(), &Ipv4Addr::new(127, 0, 0, 1));
/// assert_eq!(socket.port(), 8080);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct SocketAddrV4 {
    ip: Ipv4Addr,
    port: u16
}

/// An IPv6 socket address.
///
/// IPv6 socket addresses consist of an [Ipv6 address], a 16-bit port number, as well
/// as fields containing the traffic class, the flow label, and a scope identifier
/// (see [IETF RFC 2553, Section 3.3] for more details).
///
/// See [`SocketAddr`] for a type encompassing both IPv4 and IPv6 socket addresses.
///
/// The size of a `SocketAddrV6` struct may vary depending on the target operating
/// system.
///
/// [IETF RFC 2553, Section 3.3]: https://tools.ietf.org/html/rfc2553#section-3.3
/// [IPv6 address]: ../../std/net/struct.Ipv6Addr.html
/// [`SocketAddr`]: ../../std/net/enum.SocketAddr.html
///
/// # Examples
///
/// ```
/// use std::net::{Ipv6Addr, SocketAddrV6};
///
/// let socket = SocketAddrV6::new(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
///
/// assert_eq!("[2001:db8::1]:8080".parse(), Ok(socket));
/// assert_eq!(socket.ip(), &Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
/// assert_eq!(socket.port(), 8080);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[stable(feature = "rust1", since = "1.0.0")]
pub struct SocketAddrV6 {
    ip: Ipv6Addr,
    port: u16,
    flowinfo: u32,
    scope_id: u32,
}

impl SocketAddr {
    /// Creates a new socket address from an [IP address] and a port number.
    ///
    /// [IP address]: ../../std/net/enum.IpAddr.html
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    ///
    /// let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// assert_eq!(socket.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    /// assert_eq!(socket.port(), 8080);
    /// ```
    #[stable(feature = "ip_addr", since = "1.7.0")]
    pub fn new(ip: IpAddr, port: u16) -> SocketAddr {
        match ip {
            IpAddr::V4(a) => SocketAddr::V4(SocketAddrV4::new(a, port)),
            IpAddr::V6(a) => SocketAddr::V6(SocketAddrV6::new(a, port, 0, 0)),
        }
    }

    /// Returns the IP address associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    ///
    /// let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// assert_eq!(socket.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    /// ```
    #[stable(feature = "ip_addr", since = "1.7.0")]
    pub fn ip(&self) -> IpAddr {
        match *self {
            SocketAddr::V4(ref a) => IpAddr::V4(*a.ip()),
            SocketAddr::V6(ref a) => IpAddr::V6(*a.ip()),
        }
    }

    /// Changes the IP address associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    ///
    /// let mut socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// socket.set_ip(IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)));
    /// assert_eq!(socket.ip(), IpAddr::V4(Ipv4Addr::new(10, 10, 0, 1)));
    /// ```
    #[stable(feature = "sockaddr_setters", since = "1.9.0")]
    pub fn set_ip(&mut self, new_ip: IpAddr) {
        // `match (*self, new_ip)` would have us mutate a copy of self only to throw it away.
        match (self, new_ip) {
            (&mut SocketAddr::V4(ref mut a), IpAddr::V4(new_ip)) => a.set_ip(new_ip),
            (&mut SocketAddr::V6(ref mut a), IpAddr::V6(new_ip)) => a.set_ip(new_ip),
            (self_, new_ip) => *self_ = Self::new(new_ip, self_.port()),
        }
    }

    /// Returns the port number associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    ///
    /// let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// assert_eq!(socket.port(), 8080);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn port(&self) -> u16 {
        match *self {
            SocketAddr::V4(ref a) => a.port(),
            SocketAddr::V6(ref a) => a.port(),
        }
    }

    /// Changes the port number associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    ///
    /// let mut socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// socket.set_port(1025);
    /// assert_eq!(socket.port(), 1025);
    /// ```
    #[stable(feature = "sockaddr_setters", since = "1.9.0")]
    pub fn set_port(&mut self, new_port: u16) {
        match *self {
            SocketAddr::V4(ref mut a) => a.set_port(new_port),
            SocketAddr::V6(ref mut a) => a.set_port(new_port),
        }
    }

    /// Returns [`true`] if the [IP address] in this `SocketAddr` is an
    /// [IPv4 address], and [`false`] otherwise.
    ///
    /// [`true`]: ../../std/primitive.bool.html
    /// [`false`]: ../../std/primitive.bool.html
    /// [IP address]: ../../std/net/enum.IpAddr.html
    /// [IPv4 address]: ../../std/net/enum.IpAddr.html#variant.V4
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    ///
    /// fn main() {
    ///     let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    ///     assert_eq!(socket.is_ipv4(), true);
    ///     assert_eq!(socket.is_ipv6(), false);
    /// }
    /// ```
    #[stable(feature = "sockaddr_checker", since = "1.16.0")]
    pub fn is_ipv4(&self) -> bool {
        match *self {
            SocketAddr::V4(_) => true,
            SocketAddr::V6(_) => false,
        }
    }

    /// Returns [`true`] if the [IP address] in this `SocketAddr` is an
    /// [IPv6 address], and [`false`] otherwise.
    ///
    /// [`true`]: ../../std/primitive.bool.html
    /// [`false`]: ../../std/primitive.bool.html
    /// [IP address]: ../../std/net/enum.IpAddr.html
    /// [IPv6 address]: ../../std/net/enum.IpAddr.html#variant.V6
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{IpAddr, Ipv6Addr, SocketAddr};
    ///
    /// fn main() {
    ///     let socket = SocketAddr::new(
    ///                      IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 65535, 0, 1)), 8080);
    ///     assert_eq!(socket.is_ipv4(), false);
    ///     assert_eq!(socket.is_ipv6(), true);
    /// }
    /// ```
    #[stable(feature = "sockaddr_checker", since = "1.16.0")]
    pub fn is_ipv6(&self) -> bool {
        match *self {
            SocketAddr::V4(_) => false,
            SocketAddr::V6(_) => true,
        }
    }
}

impl SocketAddrV4 {
    /// Creates a new socket address from an [IPv4 address] and a port number.
    ///
    /// [IPv4 address]: ../../std/net/struct.Ipv4Addr.html
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV4, Ipv4Addr};
    ///
    /// let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn new(ip: Ipv4Addr, port: u16) -> SocketAddrV4 {
        SocketAddrV4 { ip, port }
    }

    /// Returns the IP address associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV4, Ipv4Addr};
    ///
    /// let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    /// assert_eq!(socket.ip(), &Ipv4Addr::new(127, 0, 0, 1));
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn ip(&self) -> &Ipv4Addr {
        &self.ip
    }

    /// Changes the IP address associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV4, Ipv4Addr};
    ///
    /// let mut socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    /// socket.set_ip(Ipv4Addr::new(192, 168, 0, 1));
    /// assert_eq!(socket.ip(), &Ipv4Addr::new(192, 168, 0, 1));
    /// ```
    #[stable(feature = "sockaddr_setters", since = "1.9.0")]
    pub fn set_ip(&mut self, new_ip: Ipv4Addr) {
        self.ip = new_ip;
    }

    /// Returns the port number associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV4, Ipv4Addr};
    ///
    /// let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    /// assert_eq!(socket.port(), 8080);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Changes the port number associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV4, Ipv4Addr};
    ///
    /// let mut socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    /// socket.set_port(4242);
    /// assert_eq!(socket.port(), 4242);
    /// ```
    #[stable(feature = "sockaddr_setters", since = "1.9.0")]
    pub fn set_port(&mut self, new_port: u16) {
        self.port = new_port;
    }
}

impl SocketAddrV6 {
    /// Creates a new socket address from an [IPv6 address], a 16-bit port number,
    /// and the `flowinfo` and `scope_id` fields.
    ///
    /// For more information on the meaning and layout of the `flowinfo` and `scope_id`
    /// parameters, see [IETF RFC 2553, Section 3.3].
    ///
    /// [IETF RFC 2553, Section 3.3]: https://tools.ietf.org/html/rfc2553#section-3.3
    /// [IPv6 address]: ../../std/net/struct.Ipv6Addr.html
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn new(ip: Ipv6Addr, port: u16, flowinfo: u32, scope_id: u32)
               -> SocketAddrV6 {
        SocketAddrV6 { ip, port, flowinfo, scope_id }
    }

    /// Returns the IP address associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
    /// assert_eq!(socket.ip(), &Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn ip(&self) -> &Ipv6Addr {
        &self.ip
    }

    /// Changes the IP address associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
    /// socket.set_ip(Ipv6Addr::new(76, 45, 0, 0, 0, 0, 0, 0));
    /// assert_eq!(socket.ip(), &Ipv6Addr::new(76, 45, 0, 0, 0, 0, 0, 0));
    /// ```
    #[stable(feature = "sockaddr_setters", since = "1.9.0")]
    pub fn set_ip(&mut self, new_ip: Ipv6Addr) {
        self.ip = new_ip;
    }

    /// Returns the port number associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
    /// assert_eq!(socket.port(), 8080);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Changes the port number associated with this socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 0);
    /// socket.set_port(4242);
    /// assert_eq!(socket.port(), 4242);
    /// ```
    #[stable(feature = "sockaddr_setters", since = "1.9.0")]
    pub fn set_port(&mut self, new_port: u16) {
        self.port = new_port;
    }

    /// Returns the flow information associated with this address.
    ///
    /// This information corresponds to the `sin6_flowinfo` field in C's `netinet/in.h`,
    /// as specified in [IETF RFC 2553, Section 3.3].
    /// It combines information about the flow label and the traffic class as specified
    /// in [IETF RFC 2460], respectively [Section 6] and [Section 7].
    ///
    /// [IETF RFC 2553, Section 3.3]: https://tools.ietf.org/html/rfc2553#section-3.3
    /// [IETF RFC 2460]: https://tools.ietf.org/html/rfc2460
    /// [Section 6]: https://tools.ietf.org/html/rfc2460#section-6
    /// [Section 7]: https://tools.ietf.org/html/rfc2460#section-7
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 10, 0);
    /// assert_eq!(socket.flowinfo(), 10);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn flowinfo(&self) -> u32 {
        self.flowinfo
    }

    /// Changes the flow information associated with this socket address.
    ///
    /// See the [`flowinfo`] method's documentation for more details.
    ///
    /// [`flowinfo`]: #method.flowinfo
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 10, 0);
    /// socket.set_flowinfo(56);
    /// assert_eq!(socket.flowinfo(), 56);
    /// ```
    #[stable(feature = "sockaddr_setters", since = "1.9.0")]
    pub fn set_flowinfo(&mut self, new_flowinfo: u32) {
        self.flowinfo = new_flowinfo;
    }

    /// Returns the scope ID associated with this address.
    ///
    /// This information corresponds to the `sin6_scope_id` field in C's `netinet/in.h`,
    /// as specified in [IETF RFC 2553, Section 3.3].
    ///
    /// [IETF RFC 2553, Section 3.3]: https://tools.ietf.org/html/rfc2553#section-3.3
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 78);
    /// assert_eq!(socket.scope_id(), 78);
    /// ```
    #[stable(feature = "rust1", since = "1.0.0")]
    pub fn scope_id(&self) -> u32 {
        self.scope_id
    }

    /// Changes the scope ID associated with this socket address.
    ///
    /// See the [`scope_id`] method's documentation for more details.
    ///
    /// [`scope_id`]: #method.scope_id
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::{SocketAddrV6, Ipv6Addr};
    ///
    /// let mut socket = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 78);
    /// socket.set_scope_id(42);
    /// assert_eq!(socket.scope_id(), 42);
    /// ```
    #[stable(feature = "sockaddr_setters", since = "1.9.0")]
    pub fn set_scope_id(&mut self, new_scope_id: u32) {
        self.scope_id = new_scope_id;
    }
}

#[stable(feature = "ip_from_ip", since = "1.16.0")]
impl From<SocketAddrV4> for SocketAddr {
    /// Converts a [`SocketAddrV4`] into a [`SocketAddr::V4`].
    fn from(sock4: SocketAddrV4) -> SocketAddr {
        SocketAddr::V4(sock4)
    }
}

#[stable(feature = "ip_from_ip", since = "1.16.0")]
impl From<SocketAddrV6> for SocketAddr {
    /// Converts a [`SocketAddrV6`] into a [`SocketAddr::V6`].
    fn from(sock6: SocketAddrV6) -> SocketAddr {
        SocketAddr::V6(sock6)
    }
}

#[stable(feature = "addr_from_into_ip", since = "1.17.0")]
impl<I: Into<IpAddr>> From<(I, u16)> for SocketAddr {
    /// Converts a tuple struct (Into<[`IpAddr`]>, `u16`) into a [`SocketAddr`].
    ///
    /// This conversion creates a [`SocketAddr::V4`] for a [`IpAddr::V4`]
    /// and creates a [`SocketAddr::V6`] for a [`IpAddr::V6`].
    ///
    /// `u16` is treated as port of the newly created [`SocketAddr`].
    fn from(pieces: (I, u16)) -> SocketAddr {
        SocketAddr::new(pieces.0.into(), pieces.1)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Display for SocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SocketAddr::V4(ref a) => a.fmt(f),
            SocketAddr::V6(ref a) => a.fmt(f),
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Display for SocketAddrV4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.ip(), self.port())
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Debug for SocketAddrV4 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Display for SocketAddrV6 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]:{}", self.ip(), self.port())
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl fmt::Debug for SocketAddrV6 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, fmt)
    }
}

#[cfg(all(test, not(target_os = "emscripten")))]
mod tests {
    use net::*;
    use net::test::{sa6, sa4};

    #[test]
    fn set_ip() {
        fn ip4(low: u8) -> Ipv4Addr { Ipv4Addr::new(77, 88, 21, low) }
        fn ip6(low: u16) -> Ipv6Addr { Ipv6Addr::new(0x2a02, 0x6b8, 0, 1, 0, 0, 0, low) }

        let mut v4 = SocketAddrV4::new(ip4(11), 80);
        assert_eq!(v4.ip(), &ip4(11));
        v4.set_ip(ip4(12));
        assert_eq!(v4.ip(), &ip4(12));

        let mut addr = SocketAddr::V4(v4);
        assert_eq!(addr.ip(), IpAddr::V4(ip4(12)));
        addr.set_ip(IpAddr::V4(ip4(13)));
        assert_eq!(addr.ip(), IpAddr::V4(ip4(13)));
        addr.set_ip(IpAddr::V6(ip6(14)));
        assert_eq!(addr.ip(), IpAddr::V6(ip6(14)));

        let mut v6 = SocketAddrV6::new(ip6(1), 80, 0, 0);
        assert_eq!(v6.ip(), &ip6(1));
        v6.set_ip(ip6(2));
        assert_eq!(v6.ip(), &ip6(2));

        let mut addr = SocketAddr::V6(v6);
        assert_eq!(addr.ip(), IpAddr::V6(ip6(2)));
        addr.set_ip(IpAddr::V6(ip6(3)));
        assert_eq!(addr.ip(), IpAddr::V6(ip6(3)));
        addr.set_ip(IpAddr::V4(ip4(4)));
        assert_eq!(addr.ip(), IpAddr::V4(ip4(4)));
    }

    #[test]
    fn set_port() {
        let mut v4 = SocketAddrV4::new(Ipv4Addr::new(77, 88, 21, 11), 80);
        assert_eq!(v4.port(), 80);
        v4.set_port(443);
        assert_eq!(v4.port(), 443);

        let mut addr = SocketAddr::V4(v4);
        assert_eq!(addr.port(), 443);
        addr.set_port(8080);
        assert_eq!(addr.port(), 8080);

        let mut v6 = SocketAddrV6::new(Ipv6Addr::new(0x2a02, 0x6b8, 0, 1, 0, 0, 0, 1), 80, 0, 0);
        assert_eq!(v6.port(), 80);
        v6.set_port(443);
        assert_eq!(v6.port(), 443);

        let mut addr = SocketAddr::V6(v6);
        assert_eq!(addr.port(), 443);
        addr.set_port(8080);
        assert_eq!(addr.port(), 8080);
    }

    #[test]
    fn set_flowinfo() {
        let mut v6 = SocketAddrV6::new(Ipv6Addr::new(0x2a02, 0x6b8, 0, 1, 0, 0, 0, 1), 80, 10, 0);
        assert_eq!(v6.flowinfo(), 10);
        v6.set_flowinfo(20);
        assert_eq!(v6.flowinfo(), 20);
    }

    #[test]
    fn set_scope_id() {
        let mut v6 = SocketAddrV6::new(Ipv6Addr::new(0x2a02, 0x6b8, 0, 1, 0, 0, 0, 1), 80, 0, 10);
        assert_eq!(v6.scope_id(), 10);
        v6.set_scope_id(20);
        assert_eq!(v6.scope_id(), 20);
    }

    #[test]
    fn is_v4() {
        let v4 = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(77, 88, 21, 11), 80));
        assert!(v4.is_ipv4());
        assert!(!v4.is_ipv6());
    }

    #[test]
    fn is_v6() {
        let v6 = SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr::new(0x2a02, 0x6b8, 0, 1, 0, 0, 0, 1), 80, 10, 0));
        assert!(!v6.is_ipv4());
        assert!(v6.is_ipv6());
    }
}
