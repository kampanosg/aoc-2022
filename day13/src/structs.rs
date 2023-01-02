use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
    Bit(u8),
    Packets(Vec<Packet>),
}

#[derive(Debug, Clone)]
pub struct Pair {
    pub packet1: Packet,
    pub packet2: Packet,
}

impl Pair {
    pub fn is_sorted(&self) -> bool {
        self.packet1 < self.packet2
    }
}

impl IntoIterator for Pair {
    type Item = Packet;
    type IntoIter = std::array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.packet1, self.packet2].into_iter()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::*;
        match (self, other) {
            (Bit(i1), Bit(i2)) => i1.cmp(i2),

            (Bit(i), Packets(_)) => Packets(vec![Bit(*i)]).cmp(other),
            (Packets(_), Bit(i)) => self.cmp(&Packets(vec![Bit(*i)])),

            (Packets(l1), Packets(l2)) => {
                for (first, second) in l1.iter().zip(l2.iter()) {
                    let result = first.cmp(second);
                    let Ordering::Equal = result else { return result; };
                }
                l1.len().cmp(&l2.len())
            }
        }
    }
}
