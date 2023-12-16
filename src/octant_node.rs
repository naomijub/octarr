
pub struct OctNode<T> {
    pub data: T,
    pub sub_nodes: Vec<OctNode<T>>,
    pub position: [i128; 3],
    pub size: u64,
}

impl OctNode<T> {
    pub fn is_leaf(&self) -> bool {
        self.sub_nodes.is_empty()
    }

    pub fn half_size(&self) -> u64 {
        self.size / 2
    }
}

