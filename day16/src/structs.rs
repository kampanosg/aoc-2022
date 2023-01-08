use std::fmt;

#[derive(Clone, Debug)]
pub struct Valve {
    pub name: ValveId,
    pub flow_rate: u64,
    pub adjacent_valves: Vec<ValveId>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValveId {
    ch1: u8,
    ch2: u8,
}

pub type Tunnel = Vec<(ValveId, ValveId)>;

impl ValveId {
    pub fn new(s: &str) -> Self {
        let b = s.as_bytes();
        Self {
            ch1: b[0],
            ch2: b[1],
        }
    }
}

impl fmt::Debug for ValveId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c1 = self.ch1 as char;
        let c2 = self.ch2 as char;
        write!(f, "{}{}", c1, c2)
    }
}
