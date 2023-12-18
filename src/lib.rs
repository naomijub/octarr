use octant_node::OctNode;

pub mod octant_id;
pub mod octant_node;

pub struct Octarr<T: Send + Clone> {
    root: OctNode<T>,
}

impl<T: Send + Clone> Default for Octarr<T> {
    fn default() -> Self {
        Self {
            root: OctNode {
                data: None,
                sub_nodes: Vec::new(),
                position: [-1, -1, -1],
                size: 2,
            },
        }
    }
}

impl<T: Send + Clone> Octarr<T> {
    // TODO: change this to `Option<&T>`
    pub fn get(&self, position: [i128; 3]) -> Option<T> {
        if self.is_outside_bounds(&position) {
            return None;
        }

        let mut current_node = &self.root;
        while current_node.size > 1 {
            if current_node.is_leaf() {
                return None;
            }
            let oct_id = current_node.get_octant_at(&position);
            if let Some(sub_node) = current_node.sub_nodes.get(oct_id.to_numeral()) {
                current_node = sub_node;
            }
        }

        current_node.data.clone()
    }

    pub fn is_outside_bounds(&self, position: &[i128; 3]) -> bool {
        let more = position[0] >= self.root.half_size() as i128
            || position[1] >= self.root.half_size() as i128
            || position[2] >= self.root.half_size() as i128;
        let less = position[0] < -(self.root.half_size() as i128)
            || position[1] < -(self.root.half_size() as i128)
            || position[2] < -(self.root.half_size() as i128);

        more || less
    }
}
