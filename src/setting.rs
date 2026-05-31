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
        let slider_y = 270.0;

        draw_text("Speed", slider_x, slider_y - 10.0, 30.0, WHITE);

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
    }
}