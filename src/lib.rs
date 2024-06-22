use octant_id::OctantId;
use octant_node::OctNode;

pub mod octant_id;
pub mod octant_node;
pub type Point = [i128;3];

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
    pub fn len(&self) -> usize {
        1 + self.root.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

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

    pub fn set(&mut self, x: i128, y: i128, z: i128, value: T) {
        while self.is_outside_bounds(&[x, y, z]) {
            self.grow();
        }

        let mut current_node = &mut self.root;
        while current_node.size > 1 {
            let octant = current_node.get_octant_at(&[x, y, z]);
            if current_node.is_leaf() {
                current_node.subdivide(1);
            }
            if current_node.sub_nodes.len() > octant.to_numeral() {
                current_node = &mut current_node.sub_nodes[octant.to_numeral()];
            }
        }

        current_node.data = Some(value);
        // self.optimize();
    }

    pub const fn is_outside_bounds(&self, position: &[i128; 3]) -> bool {
        let more = position[0] >= self.root.half_size() as i128
            || position[1] >= self.root.half_size() as i128
            || position[2] >= self.root.half_size() as i128;
        let less = position[0] < -(self.root.half_size() as i128)
            || position[1] < -(self.root.half_size() as i128)
            || position[2] < -(self.root.half_size() as i128);

        more || less
    }

    pub fn grow(&mut self) {
        self.root.subdivide(1);
        let old_nodes = self.root.sub_nodes.clone();

        let new_pos = [
            self.root.position[0] - self.root.half_size() as i128,
            self.root.position[1] - self.root.half_size() as i128,
            self.root.position[2] - self.root.half_size() as i128,
        ];
        let new_node = self.root.new(new_pos, 2);
        self.root = new_node;
        self.root.subdivide(2);

        for i in 0..8 {
            let octant = OctantId::with_index(i);
			if let Some(sub_node) = self.root.sub_nodes.get_mut(octant.to_numeral()) {
                if let Some(node) = sub_node.sub_nodes.get_mut(octant.inverse().to_numeral()) {
                    if let Some(old_node) = old_nodes.get(octant.to_numeral()) {
                        *node = old_node.clone();
                    }
                }
            }
		}
    }

    pub fn shrink(&mut self, recursive: bool) {
        if self.root.len() <= 2 { return; }
    }
}