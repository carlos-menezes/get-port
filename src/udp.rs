use crate::{Ops, Port, Range};
use std::str::FromStr;
use std::{net::Ipv4Addr, net::UdpSocket};

pub struct UdpPort;
impl Ops for UdpPort {
    fn any(host: &str) -> Option<Port> {
        let r = Range::default();
        (r.min..=r.max)
            .filter(|&p| UdpPort::is_port_available(host, p))
            .nth(0)
    }

    fn in_range(host: &str, r: Range) -> Option<Port> {
        (r.min..=r.max)
            .filter(|&p| UdpPort::is_port_available(host, p))
            .nth(0)
    }

    fn from_list(host: &str, v: Vec<Port>) -> Option<Port> {
        v.into_iter()
            .filter(|&p| UdpPort::is_port_available(host, p))
            .nth(0)
    }

    fn except(host: &str, v: Vec<Port>) -> Option<Port> {
        let r = Range::default();
        (r.min..=r.max)
            .filter(|&p| !v.contains(&p) && UdpPort::is_port_available(host, p))
            .nth(0)
    }

    fn in_range_except(host: &str, r: Range, v: Vec<Port>) -> Option<Port> {
        (r.min..=r.max)
            .filter(|&p| !v.contains(&p) && UdpPort::is_port_available(host, p))
            .nth(0)
    }

    fn is_port_available(host: &str, p: Port) -> bool {
        matches!(
            UdpSocket::bind((Ipv4Addr::from_str(host).unwrap(), p)).is_ok(),
            true
        )
    }
}
