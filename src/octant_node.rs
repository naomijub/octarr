use crate::octant_id::OctantId;
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct OctNode<T: Send + Clone> {
    pub data: Option<T>,
    pub sub_nodes: Vec<OctNode<T>>,
    pub position: [i128; 3],
    pub size: u64,
}

impl<T: Send + Clone> OctNode<T> {
    pub fn new(&self, position: [i128; 3], size: u64) -> Self {
        Self {
            data: self.data.clone(),
            sub_nodes: Vec::new(),
            position,
            size,
        }
    }

    pub fn len(&self) -> usize {
        1 + self
            .sub_nodes
            .iter()
            .map(|node| node.len())
            .reduce(|acc, len| acc + len)
            .unwrap_or_default()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set(&mut self, node: &mut Self) {
        std::mem::swap(self, node);
    }

    pub fn is_leaf(&self) -> bool {
        self.sub_nodes.is_empty()
    }

    pub const fn half_size(&self) -> u64 {
        self.size / 2
    }

    pub fn subdivide(&mut self, amount: u32) {
        if self.is_leaf() && self.size > 1 {
            for i in 0..8 {
                let position = self.get_octant(&OctantId::with_index(i));
                self.sub_nodes.push(self.new(position, self.half_size()));
            }
        }

        if amount > 1 {
            self.sub_nodes
                .par_iter_mut()
                .for_each(|node| node.subdivide(amount - 1));
        }
    }

    pub fn remove_leafs(&mut self, recursive: bool) {
        if self.is_leaf() {
            return;
        }

        if recursive {
            self.sub_nodes
                .par_iter_mut()
                .for_each(|node| node.remove_leafs(true));
        }

        for node in &self.sub_nodes {
            if !node.is_leaf() || node.data.is_some() {
                return;
            }
        }

        self.sub_nodes.clear();
    }

    pub fn get_octant_at(&self, position: &[i128; 3]) -> OctantId {
        let mut octant = OctantId::new();
        for (i, pos) in position.iter().enumerate() {
            if pos > &(self.position[i] + self.half_size() as i128) {
                octant.set_true(i);
            }
        }

        octant
    }

    pub fn get_octant(&self, octant: &OctantId) -> [i128; 3] {
        [
            self.position[0] + (octant.x() as i128 * self.half_size() as i128),
            self.position[1] + (octant.y() as i128 * self.half_size() as i128),
            self.position[2] + (octant.z() as i128 * self.half_size() as i128),
        ]
    }
}
