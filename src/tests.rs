#[cfg(test)]
mod tests {
    static HOSTNAME: &str = "127.0.0.1";

    mod tcp {
        use crate::tcp::TcpPort;
        use crate::tests::tests::HOSTNAME;
        use crate::{Ops, Range};

        #[test]
        fn it_returns_any_port() {
            assert!(
                TcpPort::any(HOSTNAME).unwrap() >= Range::default().min,
                "could not return a port in default range"
            );
        }

        #[test]
        fn it_returns_port_in_range() {
            assert!(
                TcpPort::in_range(
                    HOSTNAME,
                    Range {
                        min: 5000,
                        max: 6500
                    }
                )
                .unwrap()
                    >= 5000,
                "could not return a port in range 5000 to 6500"
            );
        }

        #[test]
        fn it_returns_port_from_list() {
            assert!(
                TcpPort::from_list(HOSTNAME, vec![6000, 7000, 8000]).unwrap() > 5999,
                "could not return a port from list [6000, 7000, 8000]"
            );
        }

        #[test]
        fn it_returns_port_except() {
            assert_ne!(
                TcpPort::except(HOSTNAME, vec![Range::default().min]).unwrap(),
                Range::default().min,
                "could not return a port except 1024"
            );
        }

        #[test]
        fn it_returns_port_in_range_except() {
            assert_ne!(
                TcpPort::in_range_except(
                    HOSTNAME,
                    Range {
                        min: 6000,
                        max: 6002
                    },
                    vec![6000]
                )
                .unwrap(),
                6000,
                "could not return in range except 6000"
            );
        }
    }

    mod udp {
        use crate::tests::tests::HOSTNAME;
        use crate::udp::UdpPort;
        use crate::{Ops, Range};

        #[test]
        fn it_returns_any_port() {
            assert!(
                UdpPort::any(HOSTNAME).unwrap() >= Range::default().min,
                "could not return a port in default range"
            );
        }

        #[test]
        fn it_returns_port_in_range() {
            assert!(
                UdpPort::in_range(
                    HOSTNAME,
                    Range {
                        min: 5000,
                        max: 6500
                    }
                )
                .unwrap()
                    >= 5000,
                "could not return a port in range 5000 to 6500"
            );
        }

        #[test]
        fn it_returns_port_from_list() {
            assert!(
                UdpPort::from_list(HOSTNAME, vec![6000, 7000, 8000]).unwrap() > 5999,
                "could not return a port from list [6000, 7000, 8000]"
            );
        }

        #[test]
        fn it_returns_port_except() {
            assert_ne!(
                UdpPort::except(HOSTNAME, vec![Range::default().min]).unwrap(),
                Range::default().min,
                "could not return a port except 1024"
            );
        }

        #[test]
        fn it_returns_port_in_range_except() {
            assert_ne!(
                UdpPort::in_range_except(
                    HOSTNAME,
                    Range {
                        min: 6000,
                        max: 6002
                    },
                    vec![6000]
                )
                .unwrap(),
                6000,
                "could not return in range except 6000"
            );
        }
    }
}
