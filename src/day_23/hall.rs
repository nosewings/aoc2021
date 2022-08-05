#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct Hall<const HALL_SIZE: usize> {
    pub tiles: [Option<u32>; HALL_SIZE],
}

impl<const HALL_SIZE: usize> Hall<HALL_SIZE> {
    pub fn new() -> Self {
        Self {
            tiles: [None; HALL_SIZE],
        }
    }

    pub fn room_to_hall_index(&self, room: u32) -> u32 {
        2 * room + 2
    }

    pub fn is_room_index(&self, ix: u32) -> bool {
        ix >= 2 && ix <= self.tiles.len() as u32 - 2 && ix % 2 == 0
    }

    pub fn set(mut self, ix: u32, x: Option<u32>) -> Self {
        self.tiles[ix as usize] = x;
        self
    }
}
