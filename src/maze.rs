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

use crate::consts::maze_consts::*;
use crate::rng::generate_rand_u32;
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
            let mut i: u8 = 1;
            while i < 32 {
                arr[i as usize] = ((1u64 << 32) / i as u64) as u32;
                i += 1;
            }
            arr
        };

        pub const POPCOUNT_4BIT: [u8; 16] = [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4];

        const CELL_POS: i32 = 12;

        let mut least_cost: u32 = u32::MAX;
        let mut best_choice: u8 = 0u8;
        let mut leaderboard: [u32; 5] = [1u32; 5];

        for &cell in &neighborhood {
            let idx: usize = ((cell & IDENTITY_MASK) >> IDENTITY_SHIFT) as usize;

            unsafe {
                let ptr: *mut u32 = leaderboard.as_mut_ptr().add(idx);
                *ptr = (*ptr).saturating_add(1);
            }
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
                    - (unsafe {
                        (*POPCOUNT_4BIT.get_unchecked(nwalls as usize) as i8 - nid as i8).abs()
                    } - unsafe {
                        (*POPCOUNT_4BIT.get_unchecked(new_nwalls as usize) as i8 - nid as i8).abs()
                    })) as u8;
            }

            let identity: u8 = unsafe { *POPCOUNT_4BIT.get_unchecked(new_walls as usize) };

            let value: u32;
            unsafe {
                value = ((((*leaderboard.get_unchecked(identity as usize) << 4) as u64)
                    * (*RECIP_U32.get_unchecked(helpfulness as usize) as u64))
                    >> 32) as u32;
            }

            let lesser_mask: u32 = !((value < least_cost) as u32).wrapping_sub(1);

            least_cost = (value & lesser_mask) | (least_cost & !lesser_mask);

            best_choice = (((new_walls << WALLS_SHIFT) | (identity << IDENTITY_SHIFT))
                & lesser_mask as u8)
                | (best_choice & !lesser_mask as u8);
        }

        best_choice
    }

    #[inline(always)]
    pub fn apply_action(&mut self, action: u8, rng_chunk: u8, skipped: bool) {
        let mask: u8 = ((rng_chunk & (rng_chunk << 4)) | 14)
            & (((self.state & 1) | (skipped as u8)).wrapping_sub(1));
        self.state ^= 1 - skipped as u8; // wakeup / sleep
        self.state = (action & mask) | (self.state & !mask);
    }
    #[inline(always)]
    pub fn new() -> CellState {
        CellState { state: 0 }
    }
}
// x = even, y = odd
impl<const SIZE: usize, const LENGTH: u16> Maze<SIZE, LENGTH> {
    // anything with d-_dilated has the alternate bits filled with 1s for proper torodialism
    // and the -_masks have all 1s for the spots above where needed so they dont break shit and wrap
    #[inline(always)]
    pub fn add_x(&self, p: u32, dx_dilated: u32) -> u32 {
        let r: u32 = (p | self.x_mask).wrapping_add(dx_dilated);
        (r & self.x_mask) | (p & self.y_mask)
    }

    #[inline(always)]
    pub fn sub_x(&self, p: u32, dx_dilated: u32) -> u32 {
        let r: u32 = (p | self.x_mask).wrapping_sub(dx_dilated);
        (r & self.x_mask) | (p & self.y_mask)
    }

    #[inline(always)]
    pub fn add_y(&self, p: u32, dy_dilated: u32) -> u32 {
        let r: u32 = (p | self.y_mask).wrapping_add(dy_dilated);
        (r & self.y_mask) | (p & self.x_mask)
    }

    #[inline(always)]
    pub fn sub_y(&self, p: u32, dy_dilated: u32) -> u32 {
        let r: u32 = (p | self.y_mask).wrapping_sub(dy_dilated);
        (r & self.y_mask) | (p & self.x_mask)
    }

    /// heyyy buddyyy, so if you use anything for this which doesnt follow these rules, you get (drumroll please) UB
    /// - - -
    /// size == length**2
    ///
    /// length == power of 2
    ///
    /// bits of length == x if length = 2**x
    /// - - -
    /// this is crucial, read it and make sure you understand
    /// ---
    #[inline(always)]
    pub fn new(bits_of_length: u8, weights: [u8; 5]) -> Self {
        Maze {
            cells: [Cell::new(); SIZE],
            length: LENGTH,
            x_mask: 0x55555555 | (u32::MAX << bits_of_length),
            y_mask: 0xAAAAAAAA | (u32::MAX << (bits_of_length + 1)),
            leaderboard_weight: weights,
        }
    }
    /// hello again (or first time) so here are the rules (follow or ub, fuck you)
    /// - - -
    /// stride length is vertical.
    /// stride has to be po2.
    /// stride has to be <= length.
    /// stride has to be greater than 0.
    /// - - -
    /// READ THIS DUMBASS
    /// ---
    #[inline(always)]
    pub unsafe fn process_stride(
        &mut self,
        stride_l: u16,
        s_pos: u32,
        flip: bool,
        id: u32,
        seed: &mut u32,
    ) {
        // btw this code is a warcrime, deadass never change it unless you can promise yourself you understand it (or sell your soul)
        const Y2: u32 = dilate_16_to_u32(2, 1);
        const X2: u32 = dilate_16_to_u32(2, 0);

        const Y1: u32 = dilate_16_to_u32(1, 1);
        const X1: u32 = dilate_16_to_u32(1, 0);

        let mut neighborhood: [u8; 25] = [0; 25];

        let mut pos: u32 = self.sub_x(self.add_y(s_pos, Y2), X2);

        // setup the neighborhood
        for y in 0..5usize {
            let mut xpos_d: u32 = pos;
            for x in 0..5usize {
                *neighborhood.get_unchecked_mut((y * 5) + x) =
                    self.cells.get_unchecked(xpos_d as usize).get_root(flip);
                xpos_d = self.add_x(xpos_d, X1)
            }
            pos = self.add_y(pos, Y1);
        }
        let mut active: u32 = s_pos;
        for _ in (0..stride_l).step_by(16) {
            let rng_block_skip: u32 = generate_rand_u32(id, *seed);
            *seed += 1;
            for b in 0..4u8 {
                let rng_block_what: u32 = generate_rand_u32(id, *seed);
                *seed += 1;
                for a in 0..4u8 {
                    let n: u8 = (b << 3) + (a << 1);
                    self.cells.get_unchecked_mut(active as usize).act(
                        flip,
                        neighborhood,
                        self.leaderboard_weight,
                        (rng_block_what >> (a as u32 * 8)) as u8,
                        (((rng_block_skip >> n) & 1) == 1)
                            & (((rng_block_skip >> (n + 1)) & 1) == 1),
                    );
                    active = self.add_y(active, Y1);
                    neighborhood.rotate_left(5);
                    let mut xpos_d: u32 = pos;
                    for x in 0..5usize {
                        *neighborhood.get_unchecked_mut(20 + x) =
                            self.cells.get_unchecked(xpos_d as usize).get_root(flip);
                        xpos_d = self.add_x(xpos_d, X1)
                    }
                    pos = self.add_y(pos, Y1);
                }
            }
        }
    }
}

impl Cell {
    #[inline(always)]
    pub fn act(
        &mut self,
        flip: bool,
        neighborhood: [u8; 25],
        leaderboard_weight: [u8; 5],
        rng_chunk: u8,
        skipped: bool,
    ) {
        let a: u8 = CellState::get_best_action(neighborhood, leaderboard_weight);
        unsafe {
            self.parts
                .get_unchecked_mut(1 - flip as usize)
                .apply_action(a, rng_chunk, skipped);
        }
    }
    #[inline(always)]
    pub fn new() -> Cell {
        Cell {
            parts: [CellState::new(); 2],
        }
    }
    #[inline(always)]
    pub fn get_root(self, flip: bool) -> u8 {
        self.parts[flip as usize].state
    }
}

/// shift == 0; -> X
///
/// shift == 1; -> Y
/// - - -
/// shift > 1; -> Broken
#[inline(always)]
pub const fn dilate_16_to_u32(input: u16, shift: u8) -> u32 {
    let mut x: u32 = input as u32;
    x = (x | (x << 8)) & 0x00FF00FF;
    x = (x | (x << 4)) & 0x0F0F0F0F;
    x = (x | (x << 2)) & 0x33333333;
    x = (x | (x << 1)) & 0x55555555;
    x << shift
}

// heyyyy if your reading this, wassup, im the dev who wrote this.
// so if your struggling to understand do i have the method for you...
// git gud.
