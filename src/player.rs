use crate::position::{Direction, Position};
use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub enum PlayerAction {
    TurnNorth,
    TurnEast,
    TurnSouth,
    TurnWest,
    MoveNorth,
    MoveEast,
    MoveSouth,
    MoveWest,
    Attack,
}
pub struct Player {
    pub position: Position,
    pub next_action: Option<PlayerAction>,
    sprite: Texture2D,
}

impl Player {
    pub async fn new(x: usize, y: usize) -> Player {
        let sprite = load_texture("./src/assets/player.png").await.unwrap(); // it's ok to panic, because image path is static
        sprite.set_filter(FilterMode::Nearest); // optional: Use Nearest filtering so pixel art stays crisp when scaled
        Player {
            position: Position::new(x, y),
            next_action: None,
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
    }
}
