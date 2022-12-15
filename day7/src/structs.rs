use std::collections::HashMap;

use id_tree::NodeId;

#[derive(Debug, Clone)]
pub struct File {
    name: String,
    size: u64,
}

#[derive(Debug, Clone)]
pub struct Directory {
    pub name: String,
    pub files: HashMap<String, File>,
    pub parrent: Option<NodeId>,
}

impl Directory {
    pub fn new(name: String, parrent: Option<NodeId>) -> Self {
        Self {
            name,
            files: HashMap::new(),
            parrent,
        }
    }

    pub fn append_file(&mut self, name: &str, size: u64) {
        self.files.entry(name.to_string()).or_insert(File {
            name: name.to_string(),
            size,
        });
    }
}
