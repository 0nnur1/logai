// opcodes
const OP_AND: u64 = 0;
const OP_OR: u64 = 1;
const OP_NAND: u64 = 2;
const OP_NOR: u64 = 3;
const OP_XOR: u64 = 4;
const OP_XNOR: u64 = 5;
const OP_ADD: u64 = 6;
const OP_MULT: u64 = 7;
const OP_SUB: u64 = 8;
const OP_DIV: u64 = 9;
const OP_CON: u64 = 10;
const MAX_VALID_OPCODE: u64 = 10;

// layout shifts
const OP_SHIFT: u32 = 60;
const FIELD_A_SHIFT: u32 = 30;

// bitmasks
const OP_MASK: u64 = 0xF;
const U30_MASK: u64 = 0x3F_FF_FF_FF;
const SIGN_BIT_30: u32 = 29;
const SIGN_EXT_MASK: i32 = -0x40_00_00_00;
const U15_MASK: u64 = 0x7FFF;

// fixed-point scaling (10-bit precision = 2^10 = 1024)
const FIXED_POINT_SHIFT: u32 = 10;
const FIXED_POINT_SCALE: f32 = 1024.0;

#[derive(Debug, PartialEq, Eq)]
pub enum GenomeError {
    InvalidOpcode(u64),
    DivisionByZero,
}

pub fn do_genome(instruction: u64) -> Result<u64, GenomeError> {
    let opcode = (instruction >> OP_SHIFT) & OP_MASK;

    if opcode > MAX_VALID_OPCODE {
        return Err(GenomeError::InvalidOpcode(opcode));
    }

    match opcode {
        // LOGIC GATES (0 to 5)
        // layout: [ Opcode (4b) | Payload (30b) | Total Conn (15b) | Recv Input (15b) ]
        OP_AND | OP_OR | OP_NAND | OP_NOR | OP_XOR | OP_XNOR => {
            let payload_bits = (instruction >> FIELD_A_SHIFT) & U30_MASK;
            let total_conn = ((instruction >> 15) & U15_MASK) as u16;
            let recv_input = (instruction & U15_MASK) as u16;

            let condition_met = match opcode {
                OP_OR => recv_input > 0,
                OP_AND => recv_input >= total_conn && total_conn > 0,
                OP_NAND => !(recv_input >= total_conn && total_conn > 0),
                OP_NOR => recv_input == 0,
                OP_XOR => recv_input == 1,
                OP_XNOR => recv_input != 1,
                _ => unreachable!(),
            };

            let final_payload = if condition_met { payload_bits } else { 0 };

            // return clean layout: wipes mutable tracker counters down to 0
            let op_part = opcode << OP_SHIFT;
            let out_payload = final_payload << FIELD_A_SHIFT;
            Ok(op_part | out_payload)
        }

        // ARITHMETIC GATES (6 to 9)
        // layout: [ Opcode (4b) | Stored Int (30b) | Input Int (30b) ]
        OP_ADD | OP_MULT | OP_SUB | OP_DIV => {
            let stored_bits = (instruction >> FIELD_A_SHIFT) & U30_MASK;
            let input_bits = instruction & U30_MASK;

            let stored_val = decode_u30(stored_bits);
            let input_val = decode_u30(input_bits);

            let result_val = match opcode {
                OP_ADD => stored_val.saturating_add(input_val),
                OP_MULT => stored_val.saturating_mul(input_val),
                OP_SUB => stored_val.saturating_sub(input_val),
                OP_DIV => {
                    if input_val == 0 {
                        return Err(GenomeError::DivisionByZero);
                    }
                    stored_val.saturating_div(input_val)
                }
                _ => unreachable!(),
            };

            let op_part = opcode << OP_SHIFT;
            let new_stored_part = (encode_u30(result_val) & U30_MASK) << FIELD_A_SHIFT;
            let new_input_part = 0u64; // input is fully consumed

            Ok(op_part | new_stored_part | new_input_part)
        }

        // STRUCTURE (10) [ OP_CON ]
        // layout: [ Opcode (4b) | Buffer Int (30b) | Weight Int (30b) ]
        OP_CON => {
            let buffer_bits = (instruction >> FIELD_A_SHIFT) & U30_MASK;
            let weight_bits = instruction & U30_MASK;

            let buffer_val = decode_u30(buffer_bits);
            let weight_raw = decode_u30(weight_bits);

            // convert fixed-point 10-bit precision representation safely to float
            let weight_float = (weight_raw as f32) / FIXED_POINT_SCALE;

            // apply scaling conversion logic: buffer * weight
            let result_float = (buffer_val as f32) * weight_float;

            // convert back to integer space
            let new_buffer_val = result_float.round() as i32;

            let op_part = opcode << OP_SHIFT;
            let new_buffer_part = (encode_u30(new_buffer_val) & U30_MASK) << FIELD_A_SHIFT;
            let new_weight_part = weight_bits; // structural weight parameter is preserved

            Ok(op_part | new_buffer_part | new_weight_part)
        }

        _ => unreachable!(),
    }
}

fn decode_u30(raw_bits: u64) -> i32 {
    let mut value = (raw_bits & U30_MASK) as i32;
    if ((value >> SIGN_BIT_30) & 1) == 1 {
        value |= SIGN_EXT_MASK;
    }
    value
}

fn encode_u30(value: i32) -> u64 {
    (value as u64) & U30_MASK
}
