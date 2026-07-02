use crate::consts::maze_consts::*;
use crate::typedef::{Cell, CellState, Maze};

// cell mapping: first 4 -> walls - next 3 -> identity - next 1 -> sleeping
impl CellState {
    #[inline(always)]
    pub fn get_best_action(neighborhood: [u8; 25], leaderboard_weight: [u8; 5]) -> u8 {
        pub const REGISTER_LUT: [u8; 81] = {
            let mut table: [u8; 81] = [0u8; 81];
            let mut i: u8 = 0;
            while i < 81 {
                table[i as usize] =
                    (((i / 27) % 3) << 6) | (((i / 9) % 3) << 4) | (((i / 3) % 3) << 2) | (i % 3);
                i += 1;
            }
            table
        };

        pub const OFFSET_LUT: [i8; 4] = [-1, 5, 1, -5];

        pub const RECIP_U32: [u32; 32] = {
            let mut arr: [u32; 32] = [0u32; 32];
            let mut i: usize = 1;
            while i < 32 {
                arr[i] = ((1u64 << 32) / i as u64) as u32;
                i += 1;
            }
            arr
        };

        const CELL_POS: i32 = 12;

        let mut least_cost: u32 = u32::MAX;
        let mut best_choice: u8 = 0u8;
        let mut leaderboard: [u32; 5] = [1u32; 5];

        for &cell in &neighborhood {
            let idx = ((cell & IDENTITY_MASK) >> IDENTITY_SHIFT) as usize;
            unsafe {
                std::intrinsics::assume(idx < 5);
            }
            leaderboard[idx] = leaderboard[idx].saturating_add(1);
        }

        for i in 0..5 {
            leaderboard[i] *= leaderboard_weight[i] as u32;
        }

        for &register in &REGISTER_LUT {
            let mut new_walls: u8 = 0u8;
            let mut helpfulness: u8 = 16u8;

            for shift in (0..8u8).step_by(2).rev() {
                let target: u8 = (register >> shift) & 3;
                let is_destroy: u8 = (target == 1) as u8;
                let is_add: u8 = (target == 2) as u8;
                let bit_shift: u8 = shift >> 1;

                new_walls |= (((neighborhood[CELL_POS as usize] >> (bit_shift + WALLS_SHIFT)) & 1)
                    & (1 - is_destroy)
                    | is_add)
                    << bit_shift;

                let neighbor: u8 =
                    neighborhood[(CELL_POS + OFFSET_LUT[(shift >> 1) as usize] as i32) as usize];

                let nid: u8 = (neighbor & IDENTITY_MASK) >> IDENTITY_SHIFT;
                let nwalls: u8 = (neighbor & WALLS_MASK) >> WALLS_SHIFT;
                let new_nwalls: u8 =
                    (nwalls & !(is_destroy << (bit_shift ^ 2))) | (is_add << (bit_shift ^ 2));

                helpfulness = (helpfulness as i8
                    - ((nwalls.count_ones() as u8).abs_diff(nid) as i8
                        - (new_nwalls.count_ones() as u8).abs_diff(nid) as i8))
                    as u8;
            }

            let identity: u8 = new_walls.count_ones() as u8;

            unsafe {
                std::intrinsics::assume(helpfulness != 0);
                std::intrinsics::assume(helpfulness < 32);
            }

            let value: u32 = ((((leaderboard[identity as usize] << 4) as u64)
                * RECIP_U32[helpfulness as usize] as u64)
                >> 32) as u32;

            let lesser: u8 = (value < least_cost) as u8;
            let lesser_mask: u32 = !lesser.wrapping_sub(1) as u32;

            least_cost = (value & lesser_mask) | (least_cost & !lesser_mask);

            best_choice = (((new_walls << WALLS_SHIFT) | (identity << IDENTITY_SHIFT))
                & lesser_mask as u8)
                | (best_choice & !lesser_mask as u8);
        }

        best_choice
    }

    #[inline(always)]
    pub fn apply_action(&mut self, action: u8, rng_chunk: u8) {
        let mask: u8 = (rng_chunk & (rng_chunk << 4)) | 14;
        self.state = (action & mask) | (self.state & !mask);
    }
}
// x = even, y = odd
impl<const SIZE: usize, const LENGTH: u16> Maze<SIZE, LENGTH> {
    // anything with d-_dilated has the alternate bits filled with 1s for proper torodialism
    // and the -_masks have all 1s for the spots above where needed so they dont break shit and wrap
    #[inline(always)]
    pub fn add_x(&self, p: u32, dx_dilated: u32) -> u32 {
        let x_bits: u32 = p & self.x_mask;
        let wrapped_x: u32 = x_bits.wrapping_add(dx_dilated) & self.x_mask;
        wrapped_x | (p & !self.x_mask)
    }

    #[inline(always)]
    pub fn sub_x(&self, p: u32, dx_dilated: u32) -> u32 {
        let x_bits: u32 = p & self.x_mask;
        let wrapped_x: u32 = x_bits.wrapping_sub(dx_dilated) & self.x_mask;
        wrapped_x | (p & !self.x_mask)
    }

    #[inline(always)]
    pub fn add_y(&self, p: u32, dy_dilated: u32) -> u32 {
        let y_mask: u32 = self.x_mask << 1;
        let y_bits: u32 = p & y_mask;
        let wrapped_y: u32 = y_bits.wrapping_add(dy_dilated) & y_mask;
        wrapped_y | (p & !y_mask)
    }

    #[inline(always)]
    pub fn sub_y(&self, p: u32, dy_dilated: u32) -> u32 {
        let y_mask: u32 = self.x_mask << 1;
        let y_bits: u32 = p & y_mask;
        let wrapped_y: u32 = y_bits.wrapping_sub(dy_dilated) & y_mask;
        wrapped_y | (p & !y_mask)
    }
}

impl Cell {
    pub fn act(
        &mut self,
        flip: u8,
        neighborhood: [u8; 25],
        leaderboard_weight: [u8; 5],
        rng_chunk: u8,
    ) {
        let a: u8 = CellState::get_best_action(neighborhood, leaderboard_weight);
        unsafe { std::intrinsics::assume(flip <= 1) };
        self.parts[flip as usize].apply_action(a, rng_chunk);
    }
}
