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

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    pressure: u32,
    opened: u64,
    pos: [u16; 2],
    time: [u32; 2],
}

impl State {
    fn upper_bound(&self, best_valves: &[Vec<(usize, u32, u32)>]) -> u32 {
        let [mut max_t, mut min_t] = self.time;
        let mut opened = self.opened;
        let mut bound = self.pressure;
        'next_valve: loop {
            for (i, min_dist, f) in &best_valves[max_t as usize] {
                if opened & (1 << i) == 0 {
                    max_t -= min_dist;
                    bound += f * max_t as u32;
                    if max_t < min_t {
                        (min_t, max_t) = (max_t, min_t);
                    }
                    opened |= 1 << i;
                    continue 'next_valve;
                }
            }
            return bound;
        }
    }
}
