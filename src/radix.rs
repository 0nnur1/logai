use crate::consts::RadixConsts;
use crate::typedef::{Bitfield, Payload, PayloadPart};

impl Bitfield {
    #[inline(always)]
    pub fn new() -> Self {
        Bitfield {
            data: RadixConsts::NULL,
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.data = RadixConsts::NULL;
    }

    #[inline(always)]
    pub fn read(&self) -> u64 {
        self.data
    }

    #[inline(always)]
    pub fn write(&mut self, to: u64) {
        debug_assert!(
            to != RadixConsts::NULL,
            "Panic: attempted to write NULL value to Bitfield in debug mode"
        );
        self.data = to;
    }

    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self.data == RadixConsts::NULL
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
        self.parts[0].is_null() & self.parts[1].is_null()
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
        self.data.write(to);
    }

    #[inline(always)]
    pub fn is_null(&self) -> bool {
        self.data.is_null()
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
