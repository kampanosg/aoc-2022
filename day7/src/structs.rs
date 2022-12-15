use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct File {
    name: String,
    size: u64,
}

#[derive(Debug, Clone)]
pub struct Directory {
    name: String,
    dirs: HashMap<String, Directory>,
    files: Vec<File>,
}

impl Directory {
    pub fn new(name: String) -> Directory {
        Directory {
            name: name,
            dirs: HashMap::new(),
            files: vec![],
        }
    }

    pub fn append_dir(&mut self, name: &str) {
        self.dirs.entry(name.to_string()).or_insert(Directory::new(name.to_string()));
    }
}
