use std::sync::MutexGuard;

use macroquad::prelude::*;
use crate::ui_elements::*;
use crate::data::GameData;

pub struct Setting {
    pub data: GameData,
    pub volume_dragging: bool,
}

impl Setting {
    pub fn new() -> Self {
        Self {
            data: GameData::load(),
            volume_dragging: false,
        }
    }

    pub fn draw(&mut self) {
        let slider_w = 220.0;
        let slider_h = 55.0;
        let slider_x = 800.0 / 2.0 - slider_w / 2.0;
        let slider_y = 220.0;
        let btn_w = 220.0;
        let btn_h = 55.0;
        let red_btn_x = 300.0 /2.0 - btn_w / 2.0;
        let red_btn_y = 350.0;
        let blue_btn_x = 800.0 /2.0 - btn_w / 2.0;
        let blue_btn_y = 350.0;
        let yellow_btn_x = 1300.0 /2.0 - btn_w / 2.0;
        let yellow_btn_y = 350.0;
        let purple_btn_x = 300.0 /2.0 - btn_w / 2.0;
        let purple_btn_y = 430.0;
        let orange_btn_x = 800.0 /2.0 - btn_w / 2.0;
        let orange_btn_y = 430.0;
        let green_btn_x = 1300.0 /2.0 - btn_w / 2.0;
        let green_btn_y = 430.0;
        let text_x = 300.0 / 2.0 - slider_w / 2.0;

        let (mouse_x, mouse_y) = mouse_position();

        let red_hover = mouse_x > red_btn_x && mouse_x < red_btn_x + btn_w
            && mouse_y > red_btn_y && mouse_y < red_btn_y + btn_h;
        
        let blue_hover = mouse_x > blue_btn_x && mouse_x < blue_btn_x + btn_w
            && mouse_y > blue_btn_y && mouse_y < blue_btn_y + btn_h;

        let yellow_hover = mouse_x > yellow_btn_x && mouse_x < yellow_btn_x + btn_w
            && mouse_y > yellow_btn_y && mouse_y < yellow_btn_y + btn_h;

        let purple_hover = mouse_x > purple_btn_x && mouse_x < purple_btn_x + btn_w
            && mouse_y > purple_btn_y && mouse_y < purple_btn_y + btn_h;

        let orange_hover = mouse_x > orange_btn_x && mouse_x < orange_btn_x + btn_w
            && mouse_y > orange_btn_y && mouse_y < orange_btn_y + btn_h;
        let green_hover = mouse_x > green_btn_x && mouse_x < green_btn_x + btn_w
            && mouse_y > green_btn_y && mouse_y < green_btn_y + btn_h;

        draw_text("Speed:", text_x, slider_y - 10.0, 30.0, WHITE);
        draw_text("Color:", text_x, red_btn_y - 10.0, 30.0, WHITE);

        draw_slider(
            slider_x,
            slider_y,
            slider_w,
            slider_h,
            &mut self.data.volume,
            0.0,
            1.0,
            &mut self.volume_dragging,
        );

        if draw_button(red_btn_x, red_btn_y, btn_w, btn_h, "Red", red_hover) {
            self.data.color = "Red".to_string();
            self.data.save();
        }

        if draw_button(blue_btn_x, blue_btn_y, btn_w, btn_h, "Blue", blue_hover) {
            self.data.color = "Blue".to_string();
            self.data.save();
        }

        if draw_button(yellow_btn_x, yellow_btn_y, btn_w, btn_h, "Yellow", yellow_hover) {
            self.data.color = "Yellow".to_string();
            self.data.save();
        }

        if draw_button(purple_btn_x, purple_btn_y, btn_w, btn_h, "Purple", purple_hover) {
            self.data.color = "Purple".to_string();
            self.data.save();
        }

        if draw_button(orange_btn_x, orange_btn_y, btn_w, btn_h, "Orange", orange_hover) {
            self.data.color = "Orange".to_string();
            self.data.save();
        }

        if draw_button(green_btn_x, green_btn_y, btn_w, btn_h, "Green", green_hover) {
            self.data.color = "Green".to_string();
            self.data.save();
        }
    }
}