pub mod maze_consts {
    pub const SMALL_SIZES: [usize; 4] = [5, 10, 16, 20];
    pub const MEDIUM_SIZES: [usize; 4] = [30, 40, 50, 75];
    pub const LARGE_SIZES: [usize; 4] = [100, 150, 250, 500];

    pub const DEF_FULL: u8 = 0b11110000;
    pub const DEF_BARREN: u8 = 0b00000000;

    pub const MAZE_TYPE_VT: [u8; 2] = [DEF_BARREN, DEF_BARREN];
    pub const WALLS_MASK: u8 = 0b11110000;
    pub const GOAL_SHIFT: u8 = 3; // 0b00001000
    pub const VISITED_SHIFT: u8 = 3; //0b00001000
    pub const MOVEMENT_COST: u8 = 0b00000011;
    pub const VISITED_MASK: u8 = 0b11110111;
    pub struct prims;

    impl prims {
        pub const GOAL_SHIFT: u8 = 6;
        pub const MOVEMENT_COST: u8 = 0b00000011;
    }
}

pub mod rng_consts {
    pub const PHILOX_M: u32 = 0x9E3779B9; // Golden ratio constant
    pub const WEYL_CONSTANT: u32 = 0xBB67AE85; // Weyl additive constant
}
