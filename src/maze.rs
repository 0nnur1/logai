use crate::consts::maze_consts::{VISITED_MASK, VISITED_SHIFT};
use crate::rng::rng_x_to_y;
use crate::typedef::{Maze, Position, PrimsMaze, RawMaze, Tile};

impl<const SIZE: usize> RawMaze<SIZE> {
    #[inline(always)]
    pub(crate) fn new(state: bool) -> Self {
        Self {
            size: SIZE,
            tiles: [Tile::new(state); SIZE * SIZE],
        }
    }

    #[inline(always)]
    fn pos_to_index(&self, pos: Position) -> usize {
        let x = (pos.x as usize) % SIZE;
        let y = (pos.y as usize) % SIZE;
        x + (y * SIZE)
    }

    #[inline(always)]
    fn index_to_pos(&self, index: usize) -> Position {
        let x = (index % SIZE) as i16;
        let y = (index / SIZE) as i16;
        Position::new(x, y)
    }

    #[inline(always)]
    fn get_neighbors(&self, pos: usize) -> [usize; 4] {
        let x = pos % SIZE;
        let y = pos / SIZE;

        let north_idx = (((y + SIZE - 1) % SIZE) * SIZE) + x;
        let south_idx = (((y + 1) % SIZE) * SIZE) + x;
        let east_idx = (y * SIZE) + ((x + 1) % SIZE);
        let west_idx = (y * SIZE) + ((x + SIZE - 1) % SIZE);

        [north_idx, east_idx, south_idx, west_idx]
    }
}

impl<const SIZE: usize> PrimsMaze<SIZE> {
    pub(crate) fn make(&mut self, unique_id: u32, seed: u32) {
        const CAP: usize = 32;
        let size = SIZE * SIZE;
        let mut frontier: [u32; CAP] = [0; CAP];

        let mut len: u8 = 1;

        let root: u32 = rng_x_to_y(0, size as i32, unique_id, seed) as u32;
        self.0.tiles[root as usize].0 |= 1 << VISITED_SHIFT;

        frontier[1] = root << 2;

        let mut iter: u32 = 0;
        while len > 0 {
            iter += 1;

            let target: usize = rng_x_to_y(0, len as i32, unique_id, seed + iter) as usize + 1;

            let packed_current = frontier[target];
            let current = (packed_current >> 2) as usize;

            frontier[target] = frontier[len as usize];
            len -= 1;

            let came_from_dir = (packed_current & 0b11) as usize;

            let execution_mask = ((iter > 1) as i16).wrapping_neg() as u16;

            let opposite_dir = (came_from_dir + 2) % 4;
            let parent = self.0.get_neighbors(current)[opposite_dir];

            let current_wall_clear = (1u16 << opposite_dir) & execution_mask;
            let parent_wall_clear = (1u16 << came_from_dir) & execution_mask;

            self.0.tiles[current].0 &= !current_wall_clear;
            self.0.tiles[parent].0 &= !parent_wall_clear;

            let neighbors: [usize; 4] = self.0.get_neighbors(current);
            let start_dir: usize = rng_x_to_y(0, 4, unique_id, seed + iter * 7919) as usize;

            for i in 0..4 {
                let dir: usize = (start_dir + i) & 3;
                let neighbor = neighbors[dir];
                let visited: u8 = (((self.0.tiles[neighbor].0 >> VISITED_SHIFT) & 1) ^ 1) as u8;
                let full_len: u8 = (len < (CAP) as u8) as u8;
                let validity: u8 = visited & full_len;

                self.0.tiles[neighbor].0 |= validity << VISITED_SHIFT;

                let packed_neighbor = ((neighbor as u32) << 2) | (dir as u32);

                len += validity;
                frontier[len as usize * validity as usize] = packed_neighbor;
            }
        }
    }
}

impl<const SIZE: usize> Maze for PrimsMaze<SIZE> {
    fn new() -> Self {
        PrimsMaze(RawMaze::new(true))
    }

    fn mutate(&self, _passes: u16, _strength: u8) {}

    fn try_move(&self, pos: &mut Position, _dir: u8) {
        pos.y = pos.y.wrapping_add(2);
    }
}

impl Position {
    #[inline(always)]
    pub fn new(x: i16, y: i16) -> Self {
        Position { x, y }
    }

    #[inline(always)]
    pub fn get_neighbors(&self) -> [Position; 4] {
        [
            Position::new(self.x, self.y - 1),
            Position::new(self.x + 1, self.y),
            Position::new(self.x, self.y + 1),
            Position::new(self.x - 1, self.y),
        ]
    }
}
