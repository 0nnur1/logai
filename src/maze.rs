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
use cuda_device::{cuda_module, kernel, thread};

// cell mapping: first 4 -> walls - next 3 -> identity - next 1 -> sleeping
impl CellState {
    #[inline(always)]
    pub fn get_best_action(neighborhood: [u8; 25], leaderboard_weight: [u8; 5]) -> u8 {
        debug_assert!(leaderboard_weight.iter().all(|&x| x > 0));
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
            let idx = ((cell & IDENTITY_MASK) >> IDENTITY_SHIFT) as usize;

            debug_assert!(idx < 5);

            unsafe {
                let ptr: *mut u32 = leaderboard.as_mut_ptr().add(idx);
                *ptr = (*ptr).saturating_add(1);
            }
        }

        for i in 0..5 {
            unsafe {
                *leaderboard.get_unchecked_mut(i) *= *leaderboard_weight.get_unchecked(i) as u32;
            }
        }

        let center: u8 = unsafe { *neighborhood.get_unchecked(CELL_POS as usize) };

        for register in REGISTER_LUT {
            let mut new_walls: u8 = 0u8;
            let mut helpfulness: u8 = 16u8;

            for shift in [6u8, 4, 2, 0] {
                let target: u8 = (register >> shift) & 3;
                let is_destroy: u8 = (target == 1) as u8;
                let is_add: u8 = (target == 2) as u8;
                let bit_shift: u8 = shift >> 1;

                new_walls |= (((center >> (bit_shift + WALLS_SHIFT)) & 1) & (1 - is_destroy)
                    | is_add)
                    << bit_shift;

                let offset: i8 = unsafe { *OFFSET_LUT.get_unchecked((shift >> 1) as usize) };

                let neighbor: u8 =
                    unsafe { *neighborhood.get_unchecked((CELL_POS + offset as i32) as usize) };

                let nid: u8 = (neighbor & IDENTITY_MASK) >> IDENTITY_SHIFT;
                let nwalls: u8 = (neighbor & WALLS_MASK) >> WALLS_SHIFT;
                let new_nwalls: u8 =
                    (nwalls & !(is_destroy << (bit_shift ^ 2))) | (is_add << (bit_shift ^ 2));

                debug_assert!(new_walls < 16);
                debug_assert!(new_nwalls < 16);
                debug_assert!(nwalls < 16);
                helpfulness = (helpfulness as i8
                    - (unsafe {
                        (*POPCOUNT_4BIT.get_unchecked(nwalls as usize) as i8 - nid as i8).abs()
                    } - unsafe {
                        (*POPCOUNT_4BIT.get_unchecked(new_nwalls as usize) as i8 - nid as i8).abs()
                    })) as u8;
            }

            let identity: u8 = unsafe { *POPCOUNT_4BIT.get_unchecked(new_walls as usize) };

            debug_assert!(identity < 5);

            let value: u32;

            debug_assert!(helpfulness < 32);
            debug_assert_ne!(helpfulness, 0);

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
        debug_assert!(((action & IDENTITY_MASK) >> IDENTITY_SHIFT) < 5);
        debug_assert!(((action & WALLS_MASK) >> WALLS_SHIFT) < 16);
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
        let r: u32 = (p | self.y_mask).wrapping_add(dx_dilated);
        ((r & self.x_mask) | (p & self.y_mask)) & self.chopper
    }

    #[inline(always)]
    pub fn sub_x(&self, p: u32, dx_dilated: u32) -> u32 {
        let r: u32 = (p | self.y_mask).wrapping_sub(dx_dilated);
        ((r & self.x_mask) | (p & self.y_mask)) & self.chopper
    }

    #[inline(always)]
    pub fn add_y(&self, p: u32, dy_dilated: u32) -> u32 {
        let r: u32 = (p | self.x_mask).wrapping_add(dy_dilated);
        ((r & self.y_mask) | (p & self.x_mask)) & self.chopper
    }

    #[inline(always)]
    pub fn sub_y(&self, p: u32, dy_dilated: u32) -> u32 {
        let r: u32 = (p | self.x_mask).wrapping_sub(dy_dilated);
        ((r & self.y_mask) | (p & self.x_mask)) & self.chopper
    }

    /// heyyy buddyyy, so if you use anything for this which doesnt follow these rules, you get (drumroll please) UB
    /// - - -
    /// size == length**2
    ///
    /// length == power of 2
    ///
    /// length >= 16
    ///
    /// bits of length == x if length = 2**x
    ///
    /// weights != 0
    /// - - -
    /// this is crucial, read it and make sure you understand
    /// ---
    #[inline(always)]
    pub fn new(bits_of_length: u8, weights: [u8; 5]) -> Self {
        debug_assert_eq!(SIZE, LENGTH as usize * LENGTH as usize);
        debug_assert!(LENGTH.is_power_of_two());
        debug_assert_eq!(LENGTH.trailing_zeros(), bits_of_length as u32);
        debug_assert!(weights.iter().all(|&w| w > 0));
        debug_assert!(LENGTH >= 16);
        debug_assert!(LENGTH <= 1024);

        let shift: u8 = bits_of_length * 2;

        Maze {
            cells: [Cell::new(); SIZE],
            length: LENGTH,
            x_mask: 0x55555555 | (u32::MAX << shift),
            y_mask: 0xAAAAAAAA | (u32::MAX << shift),
            chopper: !(u32::MAX << shift),
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
        debug_assert!(stride_l > 0);
        debug_assert!(stride_l.is_power_of_two());
        debug_assert!(stride_l <= LENGTH);

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
    debug_assert!(shift <= 1);

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

#[cuda_module]
pub mod kernel {
    use super::*;

    #[kernel]
    #[allow(no_mangle_generic_items)]
    pub unsafe fn process_cycle<const SIZE: usize, const LENGTH: u16>(
        stride_l: u16,
        s_pos: u32,
        cycles: u16,
        maze: *mut Maze<SIZE, LENGTH>,
        total_threads: u16,
        seed: u32,
        flip: bool,
    ) {
        let idx: u16 = (thread::blockIdx_x() as u16) * (thread::blockDim_x() as u16)
            + (thread::threadIdx_x() as u16);

        if idx >= total_threads {
            return;
        } // guard agaisnt invalid threads

        let length_mask: u16 = LENGTH - 1;
        let log2_length: u16 = LENGTH.trailing_zeros() as u16;

        // how many times over LENGTH (Y-axis steps)
        let times_over: u16 = idx >> log2_length;
        // remainder (X-axis steps)
        let rem_x: u16 = idx & length_mask;

        let linear_x_offset: u16 = rem_x;
        let linear_y_offset: u16 = times_over * stride_l;

        let dilated_x: u32 = dilate_16_to_u32(linear_x_offset, 0);
        let dilated_y: u32 = dilate_16_to_u32(linear_y_offset, 1);

        let mut thread_s_pos = (*maze).add_x(s_pos, dilated_x);
        thread_s_pos = (*maze).add_y(thread_s_pos, dilated_y);

        let mut seed_val: u32 = seed;
        let mut flip_act: bool = flip;

        for _ in 0..cycles {
            (*maze).process_stride(stride_l, thread_s_pos, flip_act, idx as u32, &mut seed_val);
            flip_act = !flip_act
        }
    }
}

//todo add launcher

#[cfg(test)]
mod tests {
    use super::*;

    const LEN: u16 = 16;
    const SIZE: usize = (LEN as usize) * (LEN as usize);

    fn maze() -> Maze<SIZE, LEN> {
        Maze::new(4, [1, 2, 3, 4, 5])
    }

    #[test]
    fn dilate_basic_values() {
        assert_eq!(dilate_16_to_u32(0, 0), 0);
        assert_eq!(dilate_16_to_u32(1, 0), 0x1);
        assert_eq!(dilate_16_to_u32(2, 0), 0x4);
        assert_eq!(dilate_16_to_u32(3, 0), 0x5);

        assert_eq!(dilate_16_to_u32(1, 1), 0x2);
        assert_eq!(dilate_16_to_u32(2, 1), 0x8);
        assert_eq!(dilate_16_to_u32(3, 1), 0xA);
    }

    #[test]
    fn dilate_preserves_axis_bits() {
        for i in 0..LEN {
            let x = dilate_16_to_u32(i, 0);
            let y = dilate_16_to_u32(i, 1);

            assert_eq!(x & YMASK, 0);
            assert_eq!(y & XMASK, 0);
        }
    }

    #[test]
    fn maze_new_initializes_everything() {
        let maze = maze();

        assert_eq!(maze.length, LEN);
        assert_eq!(maze.leaderboard_weight, [1, 2, 3, 4, 5]);

        for c in maze.cells {
            assert_eq!(c.parts[0].state, 0);
            assert_eq!(c.parts[1].state, 0);
        }
    }
    #[test]
    fn x_wraps_correctly() {
        let maze = maze();

        let start = dilate_16_to_u32(LEN - 1, 0);

        let end = maze.add_x(start, dilate_16_to_u32(1, 0));

        assert_eq!(end & XMASK, 0);
    }

    #[test]
    fn y_wraps_correctly() {
        let maze = maze();

        let start = dilate_16_to_u32(LEN - 1, 1);

        let end = maze.add_y(start, dilate_16_to_u32(1, 1));

        assert_eq!(end & YMASK, 0);
    }

    #[test]
    fn cellstate_new_is_zero() {
        let c = CellState::new();
        assert_eq!(c.state, 0);
    }

    #[test]
    fn cell_new_initializes_both_buffers() {
        let c = Cell::new();

        assert_eq!(c.parts[0].state, 0);
        assert_eq!(c.parts[1].state, 0);
    }

    #[test]
    fn get_root_respects_flip() {
        let mut c = Cell::new();

        c.parts[0].state = 7;
        c.parts[1].state = 9;

        assert_eq!(c.get_root(false), 7);
        assert_eq!(c.get_root(true), 9);
    }

    #[test]
    fn apply_action_skip_preserves_action_bits() {
        let mut s = CellState::new();

        s.apply_action(0b11110010, 0xFF, true);

        assert_eq!(s.state & WALLS_MASK, 0);
    }

    #[test]
    fn apply_action_non_skip_can_modify() {
        let mut s = CellState::new();

        s.apply_action(0b10100010, 0xFF, false);

        assert_ne!(s.state, 0);
    }

    #[test]
    fn best_action_returns_valid_encoding() {
        let neighborhood = [0u8; 25];

        let action = CellState::get_best_action(neighborhood, [1, 1, 1, 1, 1]);

        let id = (action & IDENTITY_MASK) >> IDENTITY_SHIFT;
        let walls = (action & WALLS_MASK) >> WALLS_SHIFT;

        assert!(id < 5);
        assert!(walls < 16);
    }

    #[test]
    fn act_only_writes_inactive_buffer() {
        let mut c = Cell::new();

        c.parts[0].state = 10;
        c.parts[1].state = 22;

        c.act(false, [0; 25], [1, 1, 1, 1, 1], 0xFF, false);

        assert_eq!(c.parts[0].state, 10);
    }

    #[test]
    fn process_stride_executes() {
        let mut maze = maze();

        let mut seed = 0;

        unsafe {
            maze.process_stride(16, 0, false, 0, &mut seed);
        }
    }

    #[test]
    fn process_stride_is_deterministic() {
        let mut a = maze();
        let mut b = maze();

        let mut sa = 12345;
        let mut sb = 12345;

        unsafe {
            a.process_stride(16, 0, false, 0, &mut sa);
            b.process_stride(16, 0, false, 0, &mut sb);
        }

        for i in 0..SIZE {
            assert_eq!(a.cells[i].parts[0].state, b.cells[i].parts[0].state);
            assert_eq!(a.cells[i].parts[1].state, b.cells[i].parts[1].state);
        }

        assert_eq!(sa, sb);
    }

    #[test]
    fn repeated_cycles_are_deterministic() {
        let mut a = maze();
        let mut b = maze();

        let mut sa = 99;
        let mut sb = 99;

        for flip in [false, true, false, true] {
            unsafe {
                a.process_stride(16, 0, flip, 3, &mut sa);
                b.process_stride(16, 0, flip, 3, &mut sb);
            }
        }

        for i in 0..SIZE {
            assert_eq!(a.cells[i].parts[0].state, b.cells[i].parts[0].state);
            assert_eq!(a.cells[i].parts[1].state, b.cells[i].parts[1].state);
        }
    }

    #[test]
    fn no_cell_produces_invalid_state() {
        let mut maze = maze();

        let mut seed = 0;

        unsafe {
            maze.process_stride(16, 0, false, 1, &mut seed);
        }

        for cell in maze.cells {
            for part in cell.parts {
                let id = (part.state & IDENTITY_MASK) >> IDENTITY_SHIFT;
                let walls = (part.state & WALLS_MASK) >> WALLS_SHIFT;

                assert!(id < 5);
                assert!(walls < 16);
            }
        }
    }
    #[test]
    fn coordinates_never_leave_range() {
        let maze = maze();

        for x in 0..LEN {
            for y in 0..LEN {
                let p = dilate_16_to_u32(x, 0) | dilate_16_to_u32(y, 1);

                assert!((maze.add_x(p, dilate_16_to_u32(1, 0)) as usize) < SIZE);
                assert!((maze.sub_x(p, dilate_16_to_u32(1, 0)) as usize) < SIZE);
                assert!((maze.add_y(p, dilate_16_to_u32(1, 1)) as usize) < SIZE);
                assert!((maze.sub_y(p, dilate_16_to_u32(1, 1)) as usize) < SIZE);
            }
        }
    }
}
