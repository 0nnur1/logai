use crate::consts::rng_consts::{PHILOX_M, WEYL_CONSTANT};

#[inline(always)]
fn philox2x32_round(counter: u32, key: u32) -> (u32, u32) {
    let product = (counter as u64).wrapping_mul(PHILOX_M as u64);
    let hi = (product >> 32) as u32;

    (hi ^ key, key.wrapping_add(WEYL_CONSTANT))
}

#[inline(always)]
pub fn generate_rand_u32(unique_id: u32, seed: u32) -> u32 {
    let mut counter = unique_id;
    let mut key = seed;

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
