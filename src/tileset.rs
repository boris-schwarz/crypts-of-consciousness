use crate::position::Position;
use macroquad::prelude::*;

#[derive(Copy, Clone)]
enum Tile {
    Grass,
    Wall,
}

// type Grid = Vec<Vec<Tile>>;

pub struct Tileset {
    pub w: usize,
    pub h: usize,
    pub tile_size: f32,
    grid: Vec<Vec<Tile>>,
}

impl Tileset {
    pub fn new(w: usize, h: usize, tile_size: f32) -> Tileset {
        Tileset {
            w,
            h,
            tile_size,
            //grid: vec![vec![Tile::Grass; h]; w],
            grid: vec![
                vec![
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Wall,
                ],
                vec![
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                ],
                vec![
                    Tile::Wall,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Grass,
                ],
                vec![
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Wall,
                ],
                vec![
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                ],
                vec![
                    Tile::Wall,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Grass,
                ],
                vec![
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Wall,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Wall,
                    Tile::Wall,
                ],
                vec![
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                    Tile::Grass,
                ],
            ],
        }
    }

    pub fn is_walkable(&self, position: Position) -> bool {
        if position.x > self.w - 1 || position.y > self.h - 1 {
            return false;
        }
        match &self.grid[position.y][position.x] {
            Tile::Grass => true,
            _ => false,
        }
    }

    pub fn render(&self) {
        for y in 0..self.h {
            for x in 0..self.w {
                let cell: &Tile = &self.grid[y][x];
                let color = match cell {
                    Tile::Grass => GREEN,
                    Tile::Wall => DARKGRAY,
                };

                draw_rectangle(
                    x as f32 * self.tile_size + 1.0,
                    y as f32 * self.tile_size + 1.0,
                    self.tile_size - 2.0,
                    self.tile_size - 2.0,
                    color,
                );
                draw_text(
                    format!("({x},{y})"),
                    x as f32 * self.tile_size,
                    y as f32 * self.tile_size + self.tile_size / 2.0 + 5.0,
                    20.0,
                    WHITE,
                );
            }
        }
    }
}
