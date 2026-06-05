pub struct RadixConsts;
pub struct OpCodes;

impl RadixConsts {
    pub const DEFAULT: u64 = u64::MAX;
    pub const OPCODE_SHIFT: u8 = 60;
    pub const OPCODE_MASK: u64 = 0x0F;
    pub const OPCODE_WIDTH: u8 = 4;
    pub const PAYLOAD_MASK: u64 = 0x0FFF_FFFF_FFFF_FFFF;
}
impl OpCodes {
    pub const NULL: u8 = u8::MAX;
    pub const POINTER: u8 = 0;
}
