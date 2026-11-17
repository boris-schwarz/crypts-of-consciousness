use macroquad::prelude::*;
use std::time::Duration;

const INTERVAL: Duration = Duration::from_millis(40);

mod player;
mod tileset;

use player::Player;
use tileset::Tileset;

#[macroquad::main("Crypts of Consciousness")]
async fn main() {
    let tileset: Tileset = Tileset::new(4, 3, 48.0);
    let mut player: Player = Player::new(0, 0).await;
    let mut last_frame = get_time();

    request_new_screen_size(800.0, 600.0);

    loop {
        let current_frame = get_time();
        let delta = current_frame - last_frame;
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Up) {
            if tileset.is_walkable(player.position.north()) {
                player.position.move_north();
            } else {
                player.position.turn_north();
            }
        }
        if is_key_pressed(KeyCode::Right) {
            if tileset.is_walkable(player.position.east()) {
                player.position.move_east();
            } else {
                player.position.turn_east();
            }
        }
        if is_key_pressed(KeyCode::Down) {
            if tileset.is_walkable(player.position.south()) {
                player.position.move_south();
            } else {
                player.position.turn_south();
            }
        }
        if is_key_pressed(KeyCode::Left) {
            if tileset.is_walkable(player.position.west()) {
                player.position.move_west();
            } else {
                player.position.turn_west();
            }
        }

        /* if delta >= INTERVAL.as_secs_f64() {
            if active == 1 {
                calculate_next_grid(&grid_1, &mut grid_2);
                active = 2;
            } else {
                calculate_next_grid(&grid_2, &mut grid_1);
                active = 1;
            }
            r = r + 1;
            last_frame = current_frame;
        } */

        tileset.render();
        player.render();

        next_frame().await
    }
}
