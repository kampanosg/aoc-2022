
use std::{
    cmp::Ordering,
    collections::BTreeSet,
};

#[derive(PartialEq, Eq)]
pub struct Node<'a> {
    pub cost: u32,
    pub curr: &'a str,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct State<'a> {
    pub opened: BTreeSet<&'a str>,
    pub curr: &'a str,
    pub elapsed: u32,
    pub relieved: u32,
}

impl<'a> Ord for Node<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<'a> PartialOrd for Node<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
