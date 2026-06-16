#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub state: bool,
}

impl Tile {
    #[inline(always)]
    pub fn new(state: bool) -> Self {
        Self { state }
    }
}

pub struct Position {
    pub x: i16,
    pub y: i16,
}

pub struct RawMaze<const SIZE: usize> {
    pub size: usize,
    pub tiles: [Tile; SIZE * SIZE],
}

pub struct PrimsMaze<const SIZE: usize>(pub RawMaze<SIZE>);

pub trait Maze {
    fn new() -> Self;
    fn mutate(&self, passes: u16, strength: u8);
    fn try_move(&self, pos: &mut Position, dir: u8);
}
