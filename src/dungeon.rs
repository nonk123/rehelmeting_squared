use rand;

pub const WIDTH: usize = 32;
pub const HEIGHT: usize = 32;

#[derive(Copy, Clone)]
pub enum Tile {
    Empty,
    Room,
}

pub struct Dungeon {
    pub tiles: [[Tile; WIDTH]; HEIGHT],
}

impl Dungeon {
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::Empty; WIDTH]; HEIGHT],
        }
    }

    pub fn generate(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.tiles[y][x] = {
                    if rand::random() {
                        Tile::Room
                    } else {
                        Tile::Empty
                    }
                };
            }
        }
    }
}
