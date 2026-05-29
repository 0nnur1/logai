const UNULL: usize = usize::MAX;
const NULL: u64 = u64::MAX;

struct RadixNode {
    children: [usize; 2],
    payload: u64,
}

impl RadixNode {
    fn new() -> Self {
        Self {
            children: [UNULL, UNULL],
            payload: NULL,
        }
    }

    fn set_payload(&mut self, payload: u64) {
        self.payload = payload;
    }

    fn clear(&mut self) {
        self.clear_children();
        self.clear_payload();
    }
    fn clear_payload(&mut self) {
        self.payload = NULL;
    }
    fn clear_children(&mut self) {
        self.children = [UNULL, UNULL];
    }
}

struct WorldState {
    nodes: Vec<RadixNode>,
}

impl WorldState {
    fn new() -> Self {
        Self {
            nodes: vec![RadixNode::new()],
        }
    }

    fn get_start_index(&self, start: u64) -> usize {
        if start == NULL {
            0
        } else {
            start as usize
        }
    }

    fn traverse_and_apply<F>(&mut self, start: u64, mut function: F)
    where
        F: FnMut(&mut RadixNode),
    {
        let current_idx = self.get_start_index(start);
        let mut stack: Vec<usize> = vec![current_idx];

        while let Some(ustart) = stack.pop() {
            function(&mut self.nodes[ustart]);

            let children_to_wipe: [usize; 2] = self.nodes[ustart].children;
            for child_idx in children_to_wipe {
                if child_idx != UNULL {
                    stack.push(child_idx);
                }
            }
        }
    }

    // FIX: Changed 'start' from usize to u64 to match traverse_and_apply(start: u64)
    fn wipe(&mut self, start: u64) {
        self.traverse_and_apply(start, |node: &mut RadixNode| node.clear());
    }

    // FIX: Changed 'start' from usize to u64 to match traverse_and_apply(start: u64)
    fn wipe_payloads(&mut self, start: u64) {
        self.traverse_and_apply(start, |node: &mut RadixNode| node.clear_payload());
    }

    pub fn ensure_string(&mut self, start: usize, binary_string: u64, depth: u8) -> usize {
        let mut current_idx = self.get_start_index(start as u64);

        for shift in (0..depth).rev() {
            let bit = ((binary_string >> shift) & 1) as usize;
            let mut next_node_idx = self.nodes[current_idx].children[bit];

            if next_node_idx == UNULL {
                next_node_idx = self.nodes.len();
                self.nodes.push(RadixNode::new());
                self.nodes[current_idx].children[bit] = next_node_idx;
            }
            current_idx = next_node_idx;
        }

        current_idx
    }
}
