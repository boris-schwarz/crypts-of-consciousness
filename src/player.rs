use macroquad::prelude::*;

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
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
            direction: Direction::Down,
        }
    }
    pub fn move_up(&mut self) {
        self.direction = Direction::Up;
        self.y -= 1;
    }
    pub fn move_right(&mut self) {
        self.direction = Direction::Right;
        self.x += 1;
    }
    pub fn move_down(&mut self) {
        self.direction = Direction::Down;
        self.y += 1;
    }
    pub fn move_left(&mut self) {
        self.direction = Direction::Left;
        self.x -= 1;
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
}
