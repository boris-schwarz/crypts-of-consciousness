use macroquad::prelude::*;

#[derive(Copy, Clone)]
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

    // moving
    pub fn turn_north(&mut self) {
        self.direction = Direction::North;
    }
    pub fn move_north(&mut self) {
        self.turn_north();
        if self.y > 0 {
            self.y -= 1;
        }
    }
    pub fn turn_east(&mut self) {
        self.direction = Direction::East;
    }
    pub fn move_east(&mut self) {
        self.turn_east();
        self.x += 1;
    }
    pub fn turn_south(&mut self) {
        self.direction = Direction::South;
    }
    pub fn move_south(&mut self) {
        self.turn_south();
        self.y += 1;
    }
    pub fn turn_west(&mut self) {
        self.direction = Direction::West;
    }
    pub fn move_west(&mut self) {
        self.turn_west();
        if self.x > 0 {
            self.x -= 1;
        }
    }

    // getters
    pub fn north(&self) -> Position {
        let y: usize = if self.y == 0 { 0 } else { self.y - 1 };
        Position {
            x: self.x,
            y,
            direction: self.direction,
        }
    }
    pub fn east(&self) -> Position {
        Position {
            x: self.x + 1,
            y: self.y,
            direction: self.direction,
        }
    }
    pub fn south(&self) -> Position {
        Position {
            x: self.x,
            y: self.y + 1,
            direction: self.direction,
        }
    }
    pub fn west(&self) -> Position {
        let x: usize = if self.x == 0 { 0 } else { self.x - 1 };
        Position {
            x: x,
            y: self.y,
            direction: self.direction,
        }
    }
}

pub struct Player {
    pub position: Position,
    sprite: Texture2D,
}

impl Player {
    pub async fn new(x: usize, y: usize) -> Player {
        let sprite = load_texture("./src/assets/player.png").await.unwrap(); // it's ok to panic, because image path is static
        sprite.set_filter(FilterMode::Nearest); // optional: Use Nearest filtering so pixel art stays crisp when scaled
        Player {
            position: Position::new(x, y),
            sprite: sprite,
        }
    }

    pub fn render(&self) {
        let sprite_coords: (f32, f32) = match &self.position.direction {
            Direction::North => (0.0, 0.0),
            Direction::East => (48.0, 0.0),
            Direction::South => (0.0, 48.0),
            Direction::West => (48.0, 48.0),
        };
        let sprite_rect = Rect::new(sprite_coords.0, sprite_coords.1, 48.0, 48.0);
        draw_texture_ex(
            &self.sprite,
            self.position.x as f32 * 48.0,
            self.position.y as f32 * 48.0,
            WHITE,
            DrawTextureParams {
                source: Some(sprite_rect),
                dest_size: Some(Vec2::new(48.0, 48.0)),
                ..Default::default()
            },
        );
        /* draw_rectangle(
            self.position.x as f32 * 48.0 + 5.0,
            self.position.y as f32 * 48.0 + 5.0,
            48.0 - 10.0,
            48.0 - 10.0,
            ORANGE,
        ); */
    }
}
