pub struct RadixConsts;

impl RadixConsts {
    pub const NULL: u64 = u64::MAX;
    pub const OPCODE_SHIFT: u8 = 60;
    pub const OPCODE_MASK: u64 = 0x0F;
    pub const OPCODE_WIDTH: u8 = 4;
}
