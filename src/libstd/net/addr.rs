use fmt;
use hash;
use io;
use mem;
use net::{ntoh, hton, IpAddr, Ipv4Addr, Ipv6Addr};
use option;
use sys::net::netc as c;
use sys_common::{FromInner, IntoInner};
use sys_common::net::LookupHost;
use vec;
use iter;
use slice;
use convert::TryInto;

impl FromInner<c::sockaddr_in> for SocketAddrV4 {
    fn from_inner(addr: c::sockaddr_in) -> SocketAddrV4 {
        SocketAddrV4 { inner: addr }
    }
}

impl FromInner<c::sockaddr_in6> for SocketAddrV6 {
    fn from_inner(addr: c::sockaddr_in6) -> SocketAddrV6 {
        SocketAddrV6 { inner: addr }
    }
}

impl<'a> IntoInner<(*const c::sockaddr, c::socklen_t)> for &'a SocketAddr {
    fn into_inner(self) -> (*const c::sockaddr, c::socklen_t) {
        match *self {
            SocketAddr::V4(ref a) => {
                (a as *const _ as *const _, mem::size_of_val(a) as c::socklen_t)
            }
            SocketAddr::V6(ref a) => {
                (a as *const _ as *const _, mem::size_of_val(a) as c::socklen_t)
            }
        }
    }
}

/// A trait for objects which can be converted or resolved to one or more
/// [`SocketAddr`] values.
///
/// This trait is used for generic address resolution when constructing network
/// objects. By default it is implemented for the following types:
///
///  * [`SocketAddr`]: [`to_socket_addrs`] is the identity function.
///
///  * [`SocketAddrV4`], [`SocketAddrV6`], `(`[`IpAddr`]`, `[`u16`]`)`,
///    `(`[`Ipv4Addr`]`, `[`u16`]`)`, `(`[`Ipv6Addr`]`, `[`u16`]`)`:
///    [`to_socket_addrs`] constructs a [`SocketAddr`] trivially.
///
///  * `(`[`&str`]`, `[`u16`]`)`: the string should be either a string representation
///    of an [`IpAddr`] address as expected by [`FromStr`] implementation or a host
///    name.
///
///  * [`&str`]: the string should be either a string representation of a
///    [`SocketAddr`] as expected by its [`FromStr`] implementation or a string like
///    `<host_name>:<port>` pair where `<port>` is a [`u16`] value.
///
/// This trait allows constructing network objects like [`TcpStream`] or
/// [`UdpSocket`] easily with values of various types for the bind/connection
/// address. It is needed because sometimes one type is more appropriate than
/// the other: for simple uses a string like `"localhost:12345"` is much nicer
/// than manual construction of the corresponding [`SocketAddr`], but sometimes
/// [`SocketAddr`] value is *the* main source of the address, and converting it to
/// some other type (e.g., a string) just for it to be converted back to
/// [`SocketAddr`] in constructor methods is pointless.
///
/// Addresses returned by the operating system that are not IP addresses are
/// silently ignored.
///
/// [`FromStr`]: ../../std/str/trait.FromStr.html
/// [`IpAddr`]: ../../std/net/enum.IpAddr.html
/// [`Ipv4Addr`]: ../../std/net/struct.Ipv4Addr.html
/// [`Ipv6Addr`]: ../../std/net/struct.Ipv6Addr.html
/// [`SocketAddr`]: ../../std/net/enum.SocketAddr.html
/// [`SocketAddrV4`]: ../../std/net/struct.SocketAddrV4.html
/// [`SocketAddrV6`]: ../../std/net/struct.SocketAddrV6.html
/// [`&str`]: ../../std/primitive.str.html
/// [`TcpStream`]: ../../std/net/struct.TcpStream.html
/// [`to_socket_addrs`]: #tymethod.to_socket_addrs
/// [`UdpSocket`]: ../../std/net/struct.UdpSocket.html
/// [`u16`]: ../../std/primitive.u16.html
///
/// # Examples
///
/// Creating a [`SocketAddr`] iterator that yields one item:
///
/// ```
/// use std::net::{ToSocketAddrs, SocketAddr};
///
/// let addr = SocketAddr::from(([127, 0, 0, 1], 443));
/// let mut addrs_iter = addr.to_socket_addrs().unwrap();
///
/// assert_eq!(Some(addr), addrs_iter.next());
/// assert!(addrs_iter.next().is_none());
/// ```
///
/// Creating a [`SocketAddr`] iterator from a hostname:
///
/// ```no_run
/// use std::net::{SocketAddr, ToSocketAddrs};
///
/// // assuming 'localhost' resolves to 127.0.0.1
/// let mut addrs_iter = "localhost:443".to_socket_addrs().unwrap();
/// assert_eq!(addrs_iter.next(), Some(SocketAddr::from(([127, 0, 0, 1], 443))));
/// assert!(addrs_iter.next().is_none());
///
/// // assuming 'foo' does not resolve
/// assert!("foo:443".to_socket_addrs().is_err());
/// ```
///
/// Creating a [`SocketAddr`] iterator that yields multiple items:
///
/// ```
/// use std::net::{SocketAddr, ToSocketAddrs};
///
/// let addr1 = SocketAddr::from(([0, 0, 0, 0], 80));
/// let addr2 = SocketAddr::from(([127, 0, 0, 1], 443));
/// let addrs = vec![addr1, addr2];
///
/// let mut addrs_iter = (&addrs[..]).to_socket_addrs().unwrap();
///
/// assert_eq!(Some(addr1), addrs_iter.next());
/// assert_eq!(Some(addr2), addrs_iter.next());
/// assert!(addrs_iter.next().is_none());
/// ```
///
/// Attempting to create a [`SocketAddr`] iterator from an improperly formatted
/// socket address `&str` (missing the port):
///
/// ```
/// use std::io;
/// use std::net::ToSocketAddrs;
///
/// let err = "127.0.0.1".to_socket_addrs().unwrap_err();
/// assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
/// ```
///
/// [`TcpStream::connect`] is an example of an function that utilizes
/// `ToSocketAddrs` as a trait bound on its parameter in order to accept
/// different types:
///
/// ```no_run
/// use std::net::{TcpStream, Ipv4Addr};
///
/// let stream = TcpStream::connect(("127.0.0.1", 443));
/// // or
/// let stream = TcpStream::connect("127.0.0.1:443");
/// // or
/// let stream = TcpStream::connect((Ipv4Addr::new(127, 0, 0, 1), 443));
/// ```
///
/// [`TcpStream::connect`]: ../../std/net/struct.TcpStream.html#method.connect
#[stable(feature = "rust1", since = "1.0.0")]
pub trait ToSocketAddrs {
    /// Returned iterator over socket addresses which this type may correspond
    /// to.
    #[stable(feature = "rust1", since = "1.0.0")]
    type Iter: Iterator<Item=SocketAddr>;

    /// Converts this object to an iterator of resolved `SocketAddr`s.
    ///
    /// The returned iterator may not actually yield any values depending on the
    /// outcome of any resolution performed.
    ///
    /// Note that this function may block the current thread while resolution is
    /// performed.
    #[stable(feature = "rust1", since = "1.0.0")]
    fn to_socket_addrs(&self) -> io::Result<Self::Iter>;
}

#[stable(feature = "rust1", since = "1.0.0")]
impl ToSocketAddrs for SocketAddr {
    type Iter = option::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> io::Result<option::IntoIter<SocketAddr>> {
        Ok(Some(*self).into_iter())
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl ToSocketAddrs for SocketAddrV4 {
    type Iter = option::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> io::Result<option::IntoIter<SocketAddr>> {
        SocketAddr::V4(*self).to_socket_addrs()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl ToSocketAddrs for SocketAddrV6 {
    type Iter = option::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> io::Result<option::IntoIter<SocketAddr>> {
        SocketAddr::V6(*self).to_socket_addrs()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl ToSocketAddrs for (IpAddr, u16) {
    type Iter = option::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> io::Result<option::IntoIter<SocketAddr>> {
        let (ip, port) = *self;
        match ip {
            IpAddr::V4(ref a) => (*a, port).to_socket_addrs(),
            IpAddr::V6(ref a) => (*a, port).to_socket_addrs(),
        }
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl ToSocketAddrs for (Ipv4Addr, u16) {
    type Iter = option::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> io::Result<option::IntoIter<SocketAddr>> {
        let (ip, port) = *self;
        SocketAddrV4::new(ip, port).to_socket_addrs()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl ToSocketAddrs for (Ipv6Addr, u16) {
    type Iter = option::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> io::Result<option::IntoIter<SocketAddr>> {
        let (ip, port) = *self;
        SocketAddrV6::new(ip, port, 0, 0).to_socket_addrs()
    }
}

fn resolve_socket_addr(lh: LookupHost) -> io::Result<vec::IntoIter<SocketAddr>> {
    let p = lh.port();
    let v: Vec<_> = lh.map(|mut a| { a.set_port(p); a }).collect();
    Ok(v.into_iter())
}

#[stable(feature = "rust1", since = "1.0.0")]
impl ToSocketAddrs for (&str, u16) {
    type Iter = vec::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> io::Result<vec::IntoIter<SocketAddr>> {
        let (host, port) = *self;

        // try to parse the host as a regular IP address first
        if let Ok(addr) = host.parse::<Ipv4Addr>() {
            let addr = SocketAddrV4::new(addr, port);
            return Ok(vec![SocketAddr::V4(addr)].into_iter())
        }
        if let Ok(addr) = host.parse::<Ipv6Addr>() {
            let addr = SocketAddrV6::new(addr, port, 0, 0);
            return Ok(vec![SocketAddr::V6(addr)].into_iter())
        }

        resolve_socket_addr((host, port).try_into()?)
    }
}

// accepts strings like 'localhost:12345'
#[stable(feature = "rust1", since = "1.0.0")]
impl ToSocketAddrs for str {
    type Iter = vec::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> io::Result<vec::IntoIter<SocketAddr>> {
        // try to parse as a regular SocketAddr first
        if let Some(addr) = self.parse().ok() {
            return Ok(vec![addr].into_iter());
        }

        resolve_socket_addr(self.try_into()?)
    }
}

#[stable(feature = "slice_to_socket_addrs", since = "1.8.0")]
impl<'a> ToSocketAddrs for &'a [SocketAddr] {
    type Iter = iter::Cloned<slice::Iter<'a, SocketAddr>>;

    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        Ok(self.iter().cloned())
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl<T: ToSocketAddrs + ?Sized> ToSocketAddrs for &T {
    type Iter = T::Iter;
    fn to_socket_addrs(&self) -> io::Result<T::Iter> {
        (**self).to_socket_addrs()
    }
}

#[stable(feature = "string_to_socket_addrs", since = "1.16.0")]
impl ToSocketAddrs for String {
    type Iter = vec::IntoIter<SocketAddr>;
    fn to_socket_addrs(&self) -> io::Result<vec::IntoIter<SocketAddr>> {
        (&**self).to_socket_addrs()
    }
}

#[cfg(all(test, not(target_os = "emscripten")))]
mod tests {
    use net::*;
    use net::test::{tsa, sa6, sa4};

    #[test]
    fn to_socket_addr_ipaddr_u16() {
        let a = Ipv4Addr::new(77, 88, 21, 11);
        let p = 12345;
        let e = SocketAddr::V4(SocketAddrV4::new(a, p));
        assert_eq!(Ok(vec![e]), tsa((a, p)));
    }

    #[test]
    fn to_socket_addr_str_u16() {
        let a = sa4(Ipv4Addr::new(77, 88, 21, 11), 24352);
        assert_eq!(Ok(vec![a]), tsa(("77.88.21.11", 24352)));

        let a = sa6(Ipv6Addr::new(0x2a02, 0x6b8, 0, 1, 0, 0, 0, 1), 53);
        assert_eq!(Ok(vec![a]), tsa(("2a02:6b8:0:1::1", 53)));

        let a = sa4(Ipv4Addr::new(127, 0, 0, 1), 23924);
        assert!(tsa(("localhost", 23924)).unwrap().contains(&a));
    }

    #[test]
    fn to_socket_addr_str() {
        let a = sa4(Ipv4Addr::new(77, 88, 21, 11), 24352);
        assert_eq!(Ok(vec![a]), tsa("77.88.21.11:24352"));

        let a = sa6(Ipv6Addr::new(0x2a02, 0x6b8, 0, 1, 0, 0, 0, 1), 53);
        assert_eq!(Ok(vec![a]), tsa("[2a02:6b8:0:1::1]:53"));

        let a = sa4(Ipv4Addr::new(127, 0, 0, 1), 23924);
        assert!(tsa("localhost:23924").unwrap().contains(&a));
    }

    #[test]
    fn to_socket_addr_string() {
        let a = sa4(Ipv4Addr::new(77, 88, 21, 11), 24352);
        assert_eq!(Ok(vec![a]), tsa(&*format!("{}:{}", "77.88.21.11", "24352")));
        assert_eq!(Ok(vec![a]), tsa(&format!("{}:{}", "77.88.21.11", "24352")));
        assert_eq!(Ok(vec![a]), tsa(format!("{}:{}", "77.88.21.11", "24352")));

        let s = format!("{}:{}", "77.88.21.11", "24352");
        assert_eq!(Ok(vec![a]), tsa(s));
        // s has been moved into the tsa call
    }

    // FIXME: figure out why this fails on openbsd and bitrig and fix it
    #[test]
    #[cfg(not(any(windows, target_os = "openbsd", target_os = "bitrig")))]
    fn to_socket_addr_str_bad() {
        assert!(tsa("1200::AB00:1234::2552:7777:1313:34300").is_err());
    }
}
