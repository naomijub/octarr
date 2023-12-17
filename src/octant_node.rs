use crate::octant_id::OctantId;

pub struct OctNode<T> {
    pub data: T,
    pub sub_nodes: Vec<OctNode<T>>,
    pub position: [i128; 3],
    pub size: u64,
}

impl<T> OctNode<T> {
    pub fn is_leaf(&self) -> bool {
        self.sub_nodes.is_empty()
    }

    pub const fn half_size(&self) -> u64 {
        self.size / 2
    }

    pub fn subdivide(&mut self, amount: u32) {}

    pub fn remove_leafs(&mut self, recursive: bool) {}

    pub fn get_octant_at(&self, position: [i128; 3]) -> OctantId {
        todo!()
    }
}
