use core::sync::atomic::AtomicU8;

pub struct CellState {
    pub state: AtomicU8,
}
pub struct Cell {
    pub parts: [CellState; 2],
}
pub struct Maze<const u16: SIZE> {
    pub size: u32,
    pub tiles: [Cell; SIZE * SIZE],
    // turn, path, junction, air, solid, dead-end
    pub leaderboard: [[i8; 2]; 6],
}
