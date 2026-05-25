// A single node in your arena. 
// Packed tightly, this structure is tiny and sits beautifully in the CPU cache.
#[derive(Clone, Debug)]
pub struct RadixNode<V> {
    // Stores indices pointing to OTHER slots inside the arena vector.
    // We use 0xFFFF as a sentinel value representing "None" (empty branch).
    pub children: [u16; 16],
    
    // The actual data payload (e.g., Drone Location, Item Data, or Instruction)
    pub value: Option<V>,
}

pub struct BinaryRadixArena<V> {
    // The flat memory pool where all nodes live right next to each other
    pub nodes: Vec<RadixNode<V>>,
    
    // A list of deleted node indices we can reuse so memory never bloats
    pub free_list: Vec<u16>,
}

impl<V> BinaryRadixArena<V> {
    const EMPTY: u16 = 0xFFFF;

    pub fn new() -> Self {
        let mut arena = Self {
            nodes: Vec::with_capacity(32),
            free_list: Vec::new(),
        };
        // Create the root node at index 0
        arena.nodes.push(RadixNode {
            children: [Self::EMPTY; 16],
            value: None,
        });
        arena
    }

    // Helper to grab a new node slot from the free list or expand the vector
    fn allocate_node(&mut self) -> u16 {
        if let Some(idx) = self.free_list.pop() {
            // Reuse a deleted node slot
            self.nodes[idx as usize] = RadixNode {
                children: [Self::EMPTY; 16],
                value: None,
            };
            idx
        } else {
            // Allocate a brand new slot at the end of the contiguous array
            let idx = self.nodes.len() as u16;
            self.nodes.push(RadixNode {
                children: [Self::EMPTY; 16],
                value: None,
            });
            idx
        }
    }

    // Insert a value using a raw 32-bit binary key (like an Entity ID or Morton Code)
    pub fn insert(&mut self, key: u32, value: V) {
        let mut current_idx = 0;

        // Process the 32-bit key in chunks of 4 bits (8 steps total)
        // Step down from the most significant bits to the least significant bits
        for shift in (0..32).step_by(4).rev() {
            // Bitmask extraction: isolate just the 4 bits we care about right now
            // e.g., (key >> 28) & 0x0F isolates the top 4 bits as a number between 0 and 15
            let nibble = ((key >> shift) & 0x0F) as usize;

            let next_idx = self.nodes[current_idx].children[nibble];

            if next_idx == Self::EMPTY {
                // Node path doesn't exist yet! Grab a new slot from our arena
                let new_node_idx = self.allocate_node();
                // Link the current node to our brand new arena slot
                self.nodes[current_idx].children[nibble] = new_node_idx;
                current_idx = new_node_idx as usize;
            } else {
                // Path already exists, just step down the array offset
                current_idx = next_idx as usize;
            }
        }

        // We reached the leaf destination, drop the value into the arena slot
        self.nodes[current_idx].value = Some(value);
    }

    // Lookup a value using a 32-bit key. Absolute blinding speed.
    pub fn get(&self, key: u32) -> Option<&V> {
        let mut current_idx = 0;

        for shift in (0..32).step_by(4).rev() {
            let nibble = ((key >> shift) & 0x0F) as usize;
            let next_idx = self.nodes[current_idx].children[nibble];

            if next_idx == Self::EMPTY {
                return None; // Cache miss, key doesn't exist
            }
            current_idx = next_idx as usize;
        }

        self.nodes[current_idx].value.as_ref()
    }
}
