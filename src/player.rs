use macroquad::prelude::*;

pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Position {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
}
impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position {
            x,
            y,
            direction: Direction::South,
        }
    }
}
pub struct Player {
    pub position: Position,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Player {
        Player {
            position: Position::new(x, y),
        }
    }

    pub fn render(&self) {
        draw_rectangle(
            self.position.x as f32 * 48.0 + 5.0,
            self.position.y as f32 * 48.0 + 5.0,
            48.0 - 10.0,
            48.0 - 10.0,
            ORANGE,
        );
    }

    pub fn move_north(&mut self) {
        self.position.y -= 1;
    }
    pub fn move_east(&mut self) {
        self.position.x += 1;
    }
    pub fn move_south(&mut self) {
        self.position.y += 1;
    }
    pub fn move_west(&mut self) {
        self.position.x -= 1;
    }
}
