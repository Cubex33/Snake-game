mod game;
mod snake;
mod food;
mod point;
mod render;
mod menu;
mod ui_elements;
mod game_menu_action;
mod setting;
mod game_state;
mod data;

use macroquad::prelude::*;
use game::*;

#[macroquad::main("Snake")]
async fn main() {
    request_new_screen_size(800.0, 600.0);
    let mut game = Game::new();

    loop {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));
        game.update();
        game.draw();
        next_frame().await;
    }
}