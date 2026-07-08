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

#[derive(Copy, Clone)]
pub struct CellState {
    pub state: u8,
}
#[derive(Copy, Clone)]
pub struct Cell {
    pub parts: [CellState; 2],
}
/// this uses a mortan curve do NOT naively try to get data (unless you know what your doing)
/// size is equal to total cells
/// length <= 16::MAX
/// size == LENGTH*LENGTH
/// maze must be a power of 2
#[derive(Copy, Clone)]
pub struct Maze<const SIZE: usize, const LENGTH: u16> {
    pub cells: [Cell; SIZE],
    pub length: u16,
    pub x_mask: u32, // e.g., 0x5555 (dilated bits for X up to lengt
    pub leaderboard_weight: [u8; 5],
}
