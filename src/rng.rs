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

use crate::consts::rng_consts::{PHILOX_M, WEYL_CONSTANT};

#[inline(always)]
fn philox2x32_round(counter: u32, key: u32) -> (u32, u32) {
    let product: u64 = (counter as u64).wrapping_mul(PHILOX_M as u64);
    let hi: u32 = (product >> 32) as u32;

    (hi ^ key, key.wrapping_add(WEYL_CONSTANT))
}

#[inline(always)]
pub fn generate_rand_u32(unique_id: u32, seed: u32) -> u32 {
    let mut counter: u32 = unique_id;
    let mut key: u32 = seed;

    for _ in 0..10 {
        let (c, k) = philox2x32_round(counter, key);
        counter = c;
        key = k;
    }
    counter
}

#[inline(always)]
pub fn rng_x_to_y(start: i32, stop: i32, unique_id: u32, seed: u32) -> i32 {
    let range_size: u32 = stop.wrapping_sub(start) as u32;

    let rand_bits: u32 = generate_rand_u32(unique_id, seed);

    let nonzero: u32 = ((range_size != 0) as u32).wrapping_neg();

    let safe_range: u32 = range_size | (1 & nonzero.wrapping_sub(1));

    let offset: u32 = rand_bits % safe_range;

    let result: i32 = start.wrapping_add(offset as i32);

    (result & nonzero as i32) | (start & !(nonzero as i32))
}
