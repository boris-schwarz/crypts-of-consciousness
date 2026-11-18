use macroquad::prelude::*;
use std::time::Duration;

const ROUND_INTERVAL: Duration = Duration::from_millis(555);

mod player;
mod position;
mod tileset;

use player::{Player, PlayerAction};
use tileset::Tileset;

#[macroquad::main("Crypts of Consciousness")]
async fn main() {
    let tileset: Tileset = Tileset::new(12, 8, 48.0);
    let mut player: Player = Player::new(0, 0).await;
    let mut last_frame = get_time();

    request_new_screen_size(800.0, 600.0);

    loop {
        let current_frame = get_time();
        let delta = current_frame - last_frame;
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Up) || is_key_down(KeyCode::Up) {
            if tileset.is_walkable(player.position.north()) {
                player.position.turn_north();
                player.next_action = Some(PlayerAction::MoveNorth);
            } else {
                player.position.turn_north();
            }
        }
        if is_key_pressed(KeyCode::Right) || is_key_down(KeyCode::Right) {
            if tileset.is_walkable(player.position.east()) {
                player.position.turn_east();
                player.next_action = Some(PlayerAction::MoveEast);
            } else {
                player.position.turn_east();
            }
        }
        if is_key_pressed(KeyCode::Down) || is_key_down(KeyCode::Down) {
            if tileset.is_walkable(player.position.south()) {
                player.position.turn_south();
                player.next_action = Some(PlayerAction::MoveSouth);
            } else {
                player.position.turn_south();
            }
        }
        if is_key_pressed(KeyCode::Left) || is_key_down(KeyCode::Left) {
            if tileset.is_walkable(player.position.west()) {
                player.position.turn_west();
                player.next_action = Some(PlayerAction::MoveWest);
            } else {
                player.position.turn_west();
            }
        }

        // only check the delta if the player is moving, otherwise actions must be immediately
        // moving should take a short duration (and maybe small animation later)
        // also turning should not happen until the player move has happened
        if delta >= ROUND_INTERVAL.as_secs_f64() {
            match player.next_action {
                Some(action) => match action {
                    PlayerAction::MoveNorth => player.position.move_north(),
                    PlayerAction::MoveEast => player.position.move_east(),
                    PlayerAction::MoveSouth => player.position.move_south(),
                    PlayerAction::MoveWest => player.position.move_west(),
                    PlayerAction::Attack => println!("attacking..."),
                    _ => {}
                },
                None => {}
            };
            player.next_action = None;
            last_frame = current_frame;
        }

        tileset.render();
        player.render();

        next_frame().await
    }
}
