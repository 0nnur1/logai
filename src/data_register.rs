pub const UNULL: usize = usize::MAX;
pub const NULL: u64 = u64::MAX;

pub struct RadixNode {
    pub children: [usize; 2],
    pub payload: u64,
}

impl RadixNode {
    pub fn new() -> Self {
        Self {
            children: [UNULL, UNULL],
            payload: NULL,
        }
    }

    pub fn set_payload(&mut self, payload: u64) {
        self.payload = payload;
    }

    pub fn clear(&mut self) {
        self.clear_children();
        self.clear_payload();
    }

    pub fn clear_payload(&mut self) {
        self.payload = NULL;
    }

    pub fn clear_children(&mut self) {
        self.children = [UNULL, UNULL];
    }

    pub fn set_child(&mut self, child_idx: usize, child: bool) {
        self.children[child as usize] = child_idx;
    }
}

pub struct WorldState {
    nodes: Vec<RadixNode>,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            nodes: vec![RadixNode::new()],
        }
    }

    pub fn add_new_node(&mut self, target_idx: usize, child: bool) -> Result<usize, &'static str> {
        if target_idx >= self.nodes.len() {
            return Err("Target node index out of bounds");
        }

        let new_node_idx = self.nodes.len();

        if self.nodes.len() == self.nodes.capacity() {
            self.nodes.reserve(self.nodes.len());
        }

        self.nodes.push(RadixNode::new());

        if let Some(node) = self.nodes.get_mut(target_idx) {
            node.set_child(new_node_idx, child);
            Ok(new_node_idx)
        } else {
            Err("Failed to access target node safely")
        }
    }

    fn get_start_index(&self, start: u64) -> usize {
        if start == NULL {
            0
        } else {
            start as usize
        }
    }

    pub fn traverse_and_apply<F>(&mut self, start: u64, mut function: F)
    where
        F: FnMut(&mut RadixNode),
    {
        let current_idx = self.get_start_index(start);
        let mut stack: Vec<usize> = vec![current_idx];

        while let Some(ustart) = stack.pop() {
            if let Some(node) = self.nodes.get_mut(ustart) {
                let children_to_wipe = node.children;

                function(node);

                for child_idx in children_to_wipe {
                    if child_idx != UNULL {
                        stack.push(child_idx);
                    }
                }
            }
        }
    }

    pub fn wipe_payloads(&mut self, start: u64) {
        self.traverse_and_apply(start, |node| node.clear_payload());
    }

    pub fn ensure_string(
        &mut self,
        start: u64,
        binary_string: u64,
        current_depth: u8,
        target_depth: u8,
    ) -> Result<usize, &'static str> {
        if current_depth
            .checked_add(target_depth)
            .map_or(true, |total| total > 64)
        {
            return Err("requested bit range exceeds 64-bit boundary");
        }

        let mut current_idx = self.get_start_index(start);

        let start_shift = 64 - current_depth;
        let end_shift = start_shift - target_depth;

        for shift in (end_shift..start_shift).rev() {
            let bit = ((binary_string >> shift) & 1) == 1;

            let node = self
                .nodes
                .get(current_idx)
                .ok_or("Radix tree corruption: active index missing")?;

            let mut next_node_idx = node.children[bit as usize];

            if next_node_idx == UNULL {
                next_node_idx = self.add_new_node(current_idx, bit)?;
            }
            current_idx = next_node_idx;
        }

        Ok(current_idx)
    }
}
