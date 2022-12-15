#[derive(Debug, Clone)]
pub struct File {
    name: String,
    size: u64,
}

#[derive(Debug, Clone)]
pub struct Directory {
    path: String,
    children: HashMap<String, Directory>,
}
