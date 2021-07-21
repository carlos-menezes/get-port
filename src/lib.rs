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

trait Ops {
    fn any(host: &str) -> Option<Port>;
    fn in_range(host: &str, r: Range) -> Option<Port>;
    fn from_list(host: &str, v: Vec<Port>) -> Option<Port>;
    fn except(host: &str, v: Vec<Port>) -> Option<Port>;
    fn in_range_except(host: &str, r: Range, v: Vec<Port>) -> Option<Port>;
    fn is_port_available(host: &str, p: Port) -> bool;
}
