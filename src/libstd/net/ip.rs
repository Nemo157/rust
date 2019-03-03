use net::{hton, ntoh, Ipv4Addr, Ipv6Addr};
use sys::net::netc as c;
use sys_common::{FromInner, IntoInner};

impl FromInner<c::in_addr> for Ipv4Addr {
    fn from_inner(addr: c::in_addr) -> Ipv4Addr {
        Ipv4Addr::from(ntoh(addr.s_addr))
    }
}

impl FromInner<c::in6_addr> for Ipv6Addr {
    fn from_inner(addr: c::in6_addr) -> Ipv6Addr {
        Ipv6Addr::from(addr.s6_addr)
    }
}

impl IntoInner<c::in_addr> for &Ipv4Addr {
    fn into_inner(self) -> c::in_addr {
        c::in_addr {
            s_addr: hton(u32::from_be_bytes(self.octets())),
        }
    }
}

impl IntoInner<c::in6_addr> for &Ipv6Addr {
    fn into_inner(self) -> c::in6_addr {
        c::in6_addr {
            s6_addr: self.octets(),
        }
    }
}
