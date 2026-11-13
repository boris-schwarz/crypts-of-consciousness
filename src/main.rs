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
    let mut player: Player = Player::new(0, 0);
    let mut last_frame = get_time();

    request_new_screen_size(800.0, 600.0);

    loop {
        let current_frame = get_time();
        let delta = current_frame - last_frame;
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Up) {
            player.position.move_up();
        }
        if is_key_pressed(KeyCode::Right) {
            player.position.move_right();
        }
        if is_key_pressed(KeyCode::Down) {
            player.position.move_down();
        }
        if is_key_pressed(KeyCode::Left) {
            player.position.move_left();
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
