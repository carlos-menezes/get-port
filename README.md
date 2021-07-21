# get-port | [crate](https://crates.io/crates/get-port)
> Get an available TCP/UDP port

---

## Information
- [crates.io package](https://crates.io/crates/get-port)
- [Documentation](https://docs.rs/get-port/4.0.0/get_port/)

---

## Example
For more examples, check `src/tests.rs`.

```rs
// Return an available port, from the supplied range, available on localhost.

use get_port::tcp::TcpPort;
use get_port::{Ops, Range};
use get_port::udp::UdpPort;

let tcp_port = TcpPort::in_range("127.0.0.1", Range {min: 6000, max: 7000 }).unwrap();
let udp_port = UdpPort::in_range("127.0.0.1", Range {min: 8000, max: 9000 }).unwrap();
```