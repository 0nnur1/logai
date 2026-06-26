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

    pub const REGISTER_LUT: [u8; 81] = {
        let mut table = [0u8; 81];
        let mut i = 0;
        while i < 81 {
            let nn = (i / 27) % 3;
            let ee = (i / 9) % 3;
            let ss = (i / 3) % 3;
            let ww = i % 3;
            table[i as usize] = (nn << 6) | (ee << 4) | (ss << 2) | ww;
            i += 1;
        }
        table
    };
    pub const OFFSET_LUT: [i8; 4] = [-1, 5, 1, -5];
}

pub mod rng_consts {
    pub const PHILOX_M: u32 = 0x9E3779B9; // Golden ratio constant
    pub const WEYL_CONSTANT: u32 = 0xBB67AE85; // Weyl additive constant
}
