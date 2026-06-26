use crate::consts::maze_consts::*;
use crate::typedef::{Cell, CellState, Maze};
use std::sync::atomic::{AtomicU8, Ordering};

// cell mapping: first 4 -> walls - next 3 -> identity - next 1 -> sleeping
impl CellState {
    #[inline]
    fn classify(&self) -> usize {
        TILE_TYPE[(state.get_walls()) as usize] as usize
    }
    pub fn get_best_action(neighborhood: [u8; 25], leaderboard_weight: [u8; 5]) -> u8 {
        const CELL_POS: i32 = 12;
        let mut least_cost: u8 = u8::MAX;
        let mut best_choice: u8 = 0;
        let mut leaderboard: [u8; 5] = [1; 5];

        for cell in neighborhood.iter().copied() {
            let idx = ((cell & IDENTITY_MASK) >> IDENTITY_SHIFT) as usize;
            leaderboard[idx] = leaderboard[idx].saturating_add(1);
        }

        for idx in 0..5 {
            leaderboard[idx] = leaderboard[idx].wrapping_mul(leaderboard_weight[idx]);
        }

        for register in REGISTER_LUT.iter().copied() {
            let mut new_walls: u8 = 0;
            let mut helpfulness: i8 = 16;

            for shift in (0..8u8).step_by(2).rev() {
                let target: u8 = (register >> shift) & 3;
                let is_destroy: u8 = (target == 1) as u8;
                let is_add: u8 = (target == 2) as u8;
                let bit_shift: u8 = shift >> 1;

                let self_wall: u8 =
                    (neighborhood[CELL_POS as usize] >> (bit_shift + WALLS_SHIFT)) & 1;
                let updated_wall: u8 = (self_wall & (1 - is_destroy)) | is_add;
                new_walls |= updated_wall << bit_shift;

                let lut_idx: usize = ((shift >> 1) & 3) as usize;
                let offset: i8 = OFFSET_LUT[lut_idx] as i8;
                let neighbor_idx: usize = (CELL_POS + offset) as usize;
                let neighbor: u8 = neighborhood[neighbor_idx];

                let nid: u8 = (neighbor & IDENTITY_MASK) >> IDENTITY_SHIFT;
                let nwalls: u8 = (neighbor & WALLS_MASK) >> WALLS_SHIFT;
                let naction: u8 = bit_shift ^ 2;

                let init_delta: u8 = (nwalls.count_ones() as u8).abs_diff(nid);
                let new_nwalls: u8 = (nwalls & !(is_destroy << naction)) | (is_add << naction);
                let new_delta: u8 = (new_nwalls.count_ones() as u8).abs_diff(nid);

                helpfulness += (init_delta as i8) - (new_delta as i8);
            }

            let identity: u8 = new_walls.count_ones() as u8;
            let identity_cost: u8 = leaderboard[identity as usize];

            let value: u8 = (identity_cost.saturating_mul(16)).saturating_div(helpfulness)
            let lesser: u8 = (value < least_cost) as u8;
            let lesser_mask: u8 = !lesser.wrapping_sub(1);
            let packed_choice: u8 = (new_walls << WALLS_SHIFT) | (identity << IDENTITY_SHIFT);

            least_cost = (value & lesser_mask) | (least_cost & !lesser_mask);
            best_choice = (packed_choice & lesser_mask) | (best_choice & !lesser_mask);
        }
        best_choice
    }
}
