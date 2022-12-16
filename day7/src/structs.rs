use std::fmt;

use id_tree::NodeId;

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub size: u64,
}

impl File {
    pub fn new(name: String, size: u64) -> Self {
        Self { name, size }
    }
}

#[derive(Clone)]
pub struct Directory {
    pub name: String,
    pub parrent: Option<NodeId>,
}

impl Directory {
    pub fn new(name: String, parrent: Option<NodeId>) -> Self {
        Self { name, parrent }
    }
}

impl fmt::Display for Directory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/", self.name)
    }
}

impl fmt::Debug for Directory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/", self.name)
    }
}
