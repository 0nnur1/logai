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
