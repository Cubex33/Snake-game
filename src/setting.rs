use macroquad::prelude::*;
use crate::ui_elements::*;

pub struct Setting;

impl Setting
{
    pub fn draw() -> GameMenuAction
    {
        let slider_w = 220.0;
        let slider_h = 55.0;
        let slider_x = 800.0 / 2.0 - slider_w / 2.0;
        let slider_y = 270.0;

        let (mouse_x, mouse_y) = mouse_position();

        let slider_hover = mouse_x > slider_x && mouse_x < slider_x + btn_w
            && mouse_y > slider_y && mouse_y < slider_y + btn_h;
    }
}