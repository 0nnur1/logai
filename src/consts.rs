/*
 * Copyright (C) 2026 0nnur1
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://gnu.org>.
 *
 * For source code requests or inquiries, please contact:
 * Email: onnuri3412@gmail.com
 */

pub mod maze_consts {
    pub const DEF_FULL: u8 = 0b11110000;
    pub const DEF_BARREN: u8 = 0b00000000;
    pub const MAZE_TYPE_VT: [u8; 2] = [DEF_BARREN, DEF_BARREN];

    const PATH: u8 = 2;
    const JUNCTION: u8 = 1;
    const AIR: u8 = 0;
    const SOLID: u8 = 4;
    const DEAD_END: u8 = 3;

    pub const TILE_TYPE: [u8; 16] = [
        AIR,      // 0000
        DEAD_END, // 0001
        DEAD_END, // 0010
        PATH,     // 0011
        DEAD_END, // 0100
        PATH,     // 0101
        PATH,     // 0110
        JUNCTION, // 0111
        DEAD_END, // 1000
        PATH,     // 1001
        PATH,     // 1010
        JUNCTION, // 1011
        PATH,     // 1100
        JUNCTION, // 1101
        JUNCTION, // 1110
        SOLID,    // 1111
    ];

    pub const WALLS_MASK: u8 = 0b11110000;
    pub const WALLS_SHIFT: u8 = 4;

    pub const IDENTITY_MASK: u8 = 0b00001110;
    pub const IDENTITY_SHIFT: u8 = 1;

    pub const XMASK: u32 = 0x55555555;
    pub const YMASK: u32 = 0xAAAAAAAA;
}

pub mod rng_consts {
    pub const PHILOX_M: u32 = 0x9E3779B9; // Golden ratio constant
    pub const WEYL_CONSTANT: u32 = 0xBB67AE85; // Weyl additive constant
}
