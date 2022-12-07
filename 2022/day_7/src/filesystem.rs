
#[derive(Debug, Clone)]
pub enum INode {
    File { name: String, size: usize },
    Directory { name: String },
}
