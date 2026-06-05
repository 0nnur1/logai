use crate::consts::OpCodes;
use crate::consts::RadixConsts;
use crate::typedef::{Bitfield, Payload, PayloadPart, Radix, RadixNode};

impl Bitfield {
    #[inline(always)]
    pub fn new() -> Self {
        Bitfield {
            data: RadixConsts::DEFAULT,
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.data = RadixConsts::DEFAULT;
    }

    #[inline(always)]
    pub fn read(&self) -> u64 {
        self.data
    }

    #[inline(always)]
    pub fn write(&mut self, to: u64) {
        self.data = to;
    }
}

impl PayloadPart {
    #[inline(always)]
    pub fn new() -> Self {
        PayloadPart {
            data: Bitfield::new(),
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.data.reset();
    }

    #[inline(always)]
    pub fn read(&self) -> u64 {
        self.data.read()
    }

    #[inline(always)]
    pub fn write(&mut self, to: u64) {
        let current_raw = self.read();

        let opcode_bits = current_raw & ((RadixConsts::OPCODE_MASK) << RadixConsts::OPCODE_SHIFT);

        let clean_to = to & RadixConsts::PAYLOAD_MASK;

        self.data.write(opcode_bits | clean_to);
    }

    #[inline(always)]
    pub fn get_opcode(&self) -> u64 {
        (self.read() >> RadixConsts::OPCODE_SHIFT) & RadixConsts::OPCODE_MASK
    }

    #[inline(always)]
    pub fn set_opcode(&mut self, opcode: u8) {
        let current_raw = self.read();

        let cleared_raw = current_raw & !((RadixConsts::OPCODE_MASK) << RadixConsts::OPCODE_SHIFT);

        let new_opcode_bits =
            ((opcode as u64) & RadixConsts::OPCODE_MASK) << RadixConsts::OPCODE_SHIFT;
        let new_raw = cleared_raw | new_opcode_bits;

        self.write(new_raw);
    }
}

impl Payload {
    #[inline(always)]
    pub fn new() -> Self {
        Payload {
            parts: [PayloadPart::new(), PayloadPart::new()],
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.parts[0].reset();
        self.parts[1].reset();
    }

    #[inline(always)]
    pub fn read(&self, child: bool) -> u64 {
        self.parts[child as usize].read()
    }

    #[inline(always)]
    pub fn write(&mut self, to: u64, child: bool) {
        self.parts[child as usize].write(to);
    }

    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self.get_combined_opcode() == OpCodes::NULL
    }

    #[inline(always)]
    pub fn get_combined_opcode(&self) -> u8 {
        let right_op = self.parts[0].get_opcode() as u8;
        let left_op = self.parts[1].get_opcode() as u8;

        (left_op << RadixConsts::OPCODE_WIDTH) | right_op
    }

    #[inline(always)]
    pub fn set_combined_opcodes(&mut self, combined_opcodes: u8) {
        let right_opcode = combined_opcodes & RadixConsts::OPCODE_MASK as u8;
        let left_opcode =
            (combined_opcodes >> RadixConsts::OPCODE_WIDTH) & RadixConsts::OPCODE_MASK as u8;

        self.parts[0].set_opcode(right_opcode);
        self.parts[1].set_opcode(left_opcode);
    }
}

impl RadixNode {
    #[inline(always)]
    pub fn new() -> Self {
        RadixNode {
            children: Payload::new(),
            payload: Payload::new(),
        }
    }

    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self.children.is_null() | self.payload.is_null()
    }
}

impl Radix {
    #[inline(always)]
    pub fn new() -> Self {
        Radix {
            nodes: vec![RadixNode::new()],
        }
    }

    #[inline(always)]
    pub fn add_child(&mut self, idx: u64, child: bool) -> u64 {
        let new_child_idx = self.create_node();

        let target = &mut self.nodes[idx as usize];
        target.children.write(new_child_idx, child);
        target.children.set_combined_opcodes(OpCodes::POINTER);
        new_child_idx
    }

    #[inline(always)]
    pub fn create_node(&mut self) -> u64 {
        let new_node = RadixNode::new();
        self.nodes.push(new_node);
        (self.nodes.len() - 1) as u64
    }
}
