use crate::data_register::WorldState;

// logic gates
const OP_AND: u64 = 0; // 0000
const OP_OR: u64 = 1; // 0001
const OP_NAND: u64 = 2; // 0010
const OP_NOR: u64 = 3; // 0011
const OP_XOR: u64 = 4; // 0100
const OP_XNOR: u64 = 5; // 0101
                        // int for the payload paramter, u15 for total connections, u15 for amount received input

// arithmetic
const OP_ADD: u64 = 6; // 0110
const OP_MULT: u64 = 7; // 0111
const OP_SUB: u64 = 8; // 1000
const OP_DIV: u64 = 9; // 1001
                       // int for current amount stored, int for input

// structure
const OP_CON: u64 = 10; // 1010
                        // int for buffer, int for weight (shifted 10 points left)

// first 4 bits are always the opcode
// integers are stored via u30 the first bit is negative or positive, the rest is basic binary
impl WorldState {
    fn foo() {}
}
