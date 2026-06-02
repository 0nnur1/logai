use crate::consts::RadixConsts;
use crate::typedef::{Bitfield, NullExceptionError, Payload, PayloadPart};

impl Bitfield {
    pub fn new() -> Self {
        Bitfield {
            data: RadixConsts::NULL,
        }
    }

    pub fn reset(&mut self) {
        self.data = RadixConsts::NULL;
    }

    pub fn read(&self) -> u64 {
        self.data
    }

    pub fn write(&mut self, to: u64) -> Result<(), NullExceptionError> {
        if to == RadixConsts::NULL {
            return Err(NullExceptionError("cannot write NULL value to string"));
        }
        self.data = to;
        Ok(())
    }

    pub fn is_null(&self) -> bool {
        self.data == RadixConsts::NULL
    }
}

impl Payload {
    pub fn new() -> Self {
        Payload {
            parts: [PayloadPart::new(), PayloadPart::new()],
        }
    }

    pub fn reset(&mut self) {
        self.parts[0].reset();
        self.parts[1].reset();
    }

    pub fn read(&self, child: bool) -> u64 {
        self.parts[child as usize].read()
    }

    pub fn write(&mut self, to: u64, child: bool) -> Result<(), NullExceptionError> {
        // Correctly forwards the Result directly
        self.parts[child as usize].write(to)
    }

    pub fn is_null(&self) -> bool {
        self.parts[0].is_null() & self.parts[1].is_null()
    }

    pub fn get_combined_opcode(&self) -> u8 {
        let right_op: u8 = self.parts[0].get_opcode() as u8;
        let left_op: u8 = self.parts[1].get_opcode() as u8;

        (left_op << RadixConsts::OPCODE_WIDTH) | right_op
    }

    pub fn set_combined_opcodes(&mut self, combined_opcodes: u8) -> Result<(), NullExceptionError> {
        let right_opcode = combined_opcodes & 0x0F;
        let left_opcode = (combined_opcodes >> RadixConsts::OPCODE_WIDTH) & 0x0F;

        // Propagates potential write errors using ?
        self.parts[0].set_opcode(right_opcode)?;
        self.parts[1].set_opcode(left_opcode)?;

        Ok(())
    }
}

impl PayloadPart {
    pub fn new() -> Self {
        PayloadPart {
            data: Bitfield::new(),
        }
    }

    pub fn reset(&mut self) {
        self.data.reset();
    }

    pub fn read(&self) -> u64 {
        self.data.read()
    }

    pub fn write(&mut self, to: u64) -> Result<(), NullExceptionError> {
        self.data.write(to)
    }

    pub fn is_null(&self) -> bool {
        self.data.is_null()
    }

    pub fn get_opcode(&self) -> u64 {
        (self.read() >> RadixConsts::OPCODE_SHIFT) & RadixConsts::OPCODE_MASK
    }

    pub fn set_opcode(&mut self, opcode: u8) -> Result<(), NullExceptionError> {
        let current_raw = self.read();

        let cleared_raw = current_raw & !((RadixConsts::OPCODE_MASK) << RadixConsts::OPCODE_SHIFT);

        let new_opcode_bits =
            ((opcode as u64) & RadixConsts::OPCODE_MASK) << RadixConsts::OPCODE_SHIFT;
        let new_raw = cleared_raw | new_opcode_bits;

        self.write(new_raw)
    }
}
