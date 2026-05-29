use macroquad::prelude::*;
use crate::ui_elements::*;
use crate::game_menu_action::*;

pub struct Menu;

impl Menu {
    pub fn draw() -> GameMenuAction {
        let title = "SNAKE";
        let title_font_size = 80;
        let title_dims = measure_text(title, None, title_font_size, 1.0);
        draw_text(
            title,
            (800.0 - title_dims.width) / 2.0,
            120.0,
            title_font_size as f32,
            GREEN,
        );

        draw_text(
            "Classic Arcade Game",
            (800.0 - measure_text("Classic Arcade Game", None, 30, 1.0).width) / 2.0,
            180.0,
            30.0,
            GRAY,
        );

        let btn_w = 220.0;
        let btn_h = 55.0;
        let btn_x = 800.0 / 2.0 - btn_w / 2.0;
        let start_y = 270.0;
        let quit_y = 350.0;

        let (mouse_x, mouse_y) = mouse_position();

        let start_hover = mouse_x > btn_x && mouse_x < btn_x + btn_w
            && mouse_y > start_y && mouse_y < start_y + btn_h;
        let quit_hover = mouse_x > btn_x && mouse_x < btn_x + btn_w
            && mouse_y > quit_y && mouse_y < quit_y + btn_h;

        if draw_button(btn_x, start_y, btn_w, btn_h, "START GAME", start_hover) {
            return GameMenuAction::Start;
        }

        if draw_button(btn_x, quit_y, btn_w, btn_h, "QUIT", quit_hover) {
            return GameMenuAction::Quit;
        }

        draw_text(
            "or press ENTER to start",
            (800.0 - measure_text("or press ENTER to start", None, 24, 1.0).width) / 2.0,
            440.0,
            24.0,
            Color::new(0.5, 0.5, 0.5, 1.0),
        );

        if is_key_pressed(KeyCode::Enter) {
            return GameMenuAction::Start;
        }
        if is_key_pressed(KeyCode::Escape) {
            return GameMenuAction::Quit;
        }

        GameMenuAction::None
    }
}

use macroquad::prelude::*;