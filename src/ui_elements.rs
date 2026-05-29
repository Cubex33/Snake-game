use macroquad::prelude::*;

pub fn draw_button(x: f32, y: f32, w: f32, h: f32, text: &str, is_hover: bool) -> bool {
    let bg_color = if is_hover {
        Color::new(0.9, 0.5, 0.1, 1.0)
    } else {
        Color::new(0.2, 0.2, 0.25, 1.0)
    };
    draw_rectangle(x, y, w, h, bg_color);
    draw_rectangle_lines(x, y, w, h, 2.5, WHITE);

    let font_size = 30.0;
    let text_dims = measure_text(text, None, font_size as u16, 1.0);
    let text_x = x + (w - text_dims.width) / 2.0;
    let text_y = y + (h + text_dims.height) / 2.0 - text_dims.offset_y;
    draw_text(text, text_x, text_y, font_size, WHITE);

    is_hover && is_mouse_button_pressed(MouseButton::Left)
}

pub fn draw_slider(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    value: &mut f32,
    min: f32,
    max: f32,
    is_dragging: &mut bool,
) {
    let (mouse_x, mouse_y) = mouse_position();

    // hover как у кнопки
    let is_hover = mouse_x > x
        && mouse_x < x + w
        && mouse_y > y
        && mouse_y < y + h;

    let bg_color = if is_hover {
        Color::new(0.9, 0.5, 0.1, 1.0)
    } else {
        Color::new(0.2, 0.2, 0.25, 1.0)
    };

    // фон
    draw_rectangle(x, y, w, h, bg_color);
    draw_rectangle_lines(x, y, w, h, 2.5, WHITE);

    // трек
    let padding = 18.0;
    let track_x = x + padding;
    let track_y = y + h / 2.0 - 4.0;
    let track_w = w - padding * 2.0;
    let track_h = 8.0;

    draw_rectangle(
        track_x,
        track_y,
        track_w,
        track_h,
        Color::new(0.3, 0.3, 0.35, 1.0),
    );

    // заполненная часть
    let normalized = (*value - min) / (max - min);
    let filled_w = track_w * normalized;

    draw_rectangle(
        track_x,
        track_y,
        filled_w,
        track_h,
        Color::new(0.2, 0.8, 0.2, 1.0),
    );

    // кружок
    let knob_radius = 10.0;
    let knob_x = track_x + filled_w;
    let knob_y = track_y + track_h / 2.0;

    let over_knob =
        (mouse_x - knob_x).abs() < knob_radius &&
        (mouse_y - knob_y).abs() < knob_radius;

    if (over_knob || is_hover)
        && is_mouse_button_pressed(MouseButton::Left)
    {
        *is_dragging = true;
    }

    if *is_dragging {
        if is_mouse_button_released(MouseButton::Left) {
            *is_dragging = false;
        } else {
            let pos = (mouse_x - track_x).clamp(0.0, track_w);
            *value = min + (pos / track_w) * (max - min);
        }
    }

    let knob_color = if *is_dragging {
        ORANGE
    } else if over_knob {
        YELLOW
    } else {
        WHITE
    };

    draw_circle(knob_x, knob_y, knob_radius, knob_color);
    draw_circle_lines(knob_x, knob_y, knob_radius, 2.0, WHITE);
}