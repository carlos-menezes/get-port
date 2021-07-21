pub mod tcp;
mod tests;
pub mod udp;

pub struct Range {
    pub min: u16,
    pub max: u16,
}

type Port = u16;

impl Default for Range {
    fn default() -> Self {
        Range {
            min: 1024,
            max: 65535,
        }
    }
}

/// A trait for defining behaviour on the lib's functions
pub trait Ops {
    /// Returns any port in default range.
    ///
    /// # Arguments
    ///* `host` - a string slice pointing to the hostname to test the port availability on.
    ///
    /// # Examples
    ///```
    ///use get_port::tcp::TcpPort;
    ///use get_port::udp::UdpPort;
    ///use get_port::Ops;
    ///
    ///let tcp_port = TcpPort::any("127.0.0.1").unwrap();
    ///let udp_port = UdpPort::any("127.0.0.1").unwrap();
    /// ```
    fn any(host: &str) -> Option<Port>;

    /// Returns any port in given range.
    ///
    /// # Arguments
    /// * `host` - a string slice pointing to the hostname to test the port availability on.
    /// * `r` - the Range to choose a port from.
    ///
    /// # Examples
    /// ```
    /// use get_port::tcp::TcpPort;
    /// use get_port::{Ops, Range};
    /// use get_port::udp::UdpPort;
    ///
    /// let tcp_port = TcpPort::in_range("127.0.0.1", Range {min: 6000, max: 7000 }).unwrap();
    /// let udp_port = UdpPort::in_range("127.0.0.1", Range {min: 8000, max: 9000 }).unwrap();
    /// ```
    fn in_range(host: &str, r: Range) -> Option<Port>;

    /// Returns any port from the supplied list.
    ///
    /// # Arguments
    /// * `host` - a string slice pointing to the hostname to test the port availability on.
    /// * `v` - a vector of Ports (`u16`) to choose from.
    ///
    /// # Examples
    /// ```
    /// use get_port::tcp::TcpPort;
    /// use get_port::Ops;
    /// use get_port::udp::UdpPort;
    ///
    /// let tcp_port = TcpPort::from_list("127.0.0.1", vec![5000, 6000]).unwrap();
    /// let udp_port = UdpPort::from_list("127.0.0.1", vec![5000, 6000]).unwrap();
    /// ```
    fn from_list(host: &str, v: Vec<Port>) -> Option<Port>;

    /// Returns any port from the default range except for the ones in the supplied list.
    ///
    /// # Arguments
    /// * `host` - a string slice pointing to the hostname to test the port availability on.
    /// * `v` - a vector of Ports (`u16`) to exclude.
    ///
    /// # Examples
    /// ```
    /// use get_port::tcp::TcpPort;
    /// use get_port::Ops;
    /// use get_port::udp::UdpPort;
    ///
    /// let tcp_port = TcpPort::except("127.0.0.1", vec![1456, 6541]).unwrap();
    /// let udp_port = UdpPort::except("127.0.0.1", vec![1456, 6541]).unwrap();
    /// ```
    fn except(host: &str, v: Vec<Port>) -> Option<Port>;

    /// Returns any port from the supplied range except for the ones in the supplied list.
    ///
    /// # Arguments
    /// * `host` - a string slice pointing to the hostname to test the port availability on.
    /// * `r` - the Range to choose a port from.
    /// * `v` - a vector of Ports (`u16`) to exclude.
    ///
    /// # Examples
    /// ```
    /// use get_port::tcp::TcpPort;
    /// use get_port::{Ops, Range};
    /// use get_port::udp::UdpPort;
    ///
    /// let tcp_port = TcpPort::in_range_except("127.0.0.1", Range { min: 6000, max: 7000 }, vec![6500]).unwrap();
    /// let udp_port = UdpPort::in_range_except("127.0.0.1", Range { min: 6000, max: 7000 }, vec![6500]).unwrap();
    /// ```
    fn in_range_except(host: &str, r: Range, v: Vec<Port>) -> Option<Port>;

    /// Utility function to check whether a port is available or not.
    fn is_port_available(host: &str, p: Port) -> bool;
}
