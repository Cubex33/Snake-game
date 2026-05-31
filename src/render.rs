use crate::point::Point;
use crate::snake::Snake;
use crate::food::{GRID_WIDTH, GRID_HEIGHT};
use macroquad::prelude::*;

pub const CELL_SIZE: f32 = 25.0;
pub const OFFSET_X: f32 = 150.0;
pub const OFFSET_Y: f32 = 50.0;

pub fn grid_to_screen(x: i32, y: i32) -> (f32, f32) {
    (
        OFFSET_X + x as f32 * CELL_SIZE,
        OFFSET_Y + y as f32 * CELL_SIZE,
    )
}

pub fn draw_grid() {
    for x in 0..=GRID_WIDTH {
        let x_pos = OFFSET_X + x as f32 * CELL_SIZE;
        draw_line(
            x_pos,
            OFFSET_Y,
            x_pos,
            OFFSET_Y + GRID_HEIGHT as f32 * CELL_SIZE,
            0.5,
            Color::new(0.15, 0.15, 0.25, 1.0),
        );
    }
    for y in 0..=GRID_HEIGHT {
        let y_pos = OFFSET_Y + y as f32 * CELL_SIZE;
        draw_line(
            OFFSET_X,
            y_pos,
            OFFSET_X + GRID_WIDTH as f32 * CELL_SIZE,
            y_pos,
            0.5,
            Color::new(0.15, 0.15, 0.25, 1.0),
        );
    }
}

fn brighten(color: Color, amount: f32) -> Color {
    Color::new(
        (color.r + amount).min(1.0),
        (color.g + amount).min(1.0),
        (color.b + amount).min(1.0),
        color.a,
    )
}

fn darken(color: Color, amount: f32) -> Color {
    Color::new(
        (color.r - amount).max(0.0),
        (color.g - amount).max(0.0),
        (color.b - amount).max(0.0),
        color.a,
    )
}

pub fn draw_snake(snake: &Snake) {
    let head_color = brighten(snake.color, 0.2);
    let body1_color = snake.color;
    let body2_color = brighten(snake.color, 0.08);
    let border_color = darken(snake.color, 0.3);

    for (i, segment) in snake.body.iter().enumerate() {
        let (sx, sy) = grid_to_screen(segment.x, segment.y);
        let size = CELL_SIZE - 2.0;

        let color = if i == 0 {
            head_color
        } else if i % 2 == 0 {
            body1_color
        } else {
            body2_color
        };

        draw_rectangle(sx, sy, size, size, color);
        draw_rectangle_lines(sx, sy, size, size, 1.5, border_color);

        if i == 0 {
            let dir = snake.dir;

            let eye_radius = 2.5;
            let pupil_radius = 1.2;

            let head_center_x = sx + size / 2.0;
            let head_center_y = sy + size / 2.0;

            let (base_eye_offset_x, base_eye_offset_y) = match (dir.x, dir.y) {
                (1, 0) => (5.0, -4.0),
                (-1, 0) => (-5.0, -4.0),
                (0, -1) => (-4.0, -5.0),
                (0, 1) => (-4.0, 5.0),
                _ => (5.0, -4.0),
            };

            let perp_x = -dir.y as f32;
            let perp_y = dir.x as f32;

            let eye1 = (
                head_center_x + base_eye_offset_x + perp_x * 3.5,
                head_center_y + base_eye_offset_y + perp_y * 3.5,
            );

            let eye2 = (
                head_center_x + base_eye_offset_x - perp_x * 3.5,
                head_center_y + base_eye_offset_y - perp_y * 3.5,
            );

            draw_circle(eye1.0, eye1.1, eye_radius, WHITE);
            draw_circle(eye2.0, eye2.1, eye_radius, WHITE);

            draw_circle(eye1.0, eye1.1, pupil_radius, BLACK);
            draw_circle(eye2.0, eye2.1, pupil_radius, BLACK);
        }
    }
}

pub fn draw_food(food: &Point, anim_time: f32) {
    let (fx, fy) = grid_to_screen(food.x, food.y);
    let center_x = fx + CELL_SIZE / 2.0;
    let center_y = fy + CELL_SIZE / 2.0;
    let base_radius = 8.0;
    let pulse = 1.5 * f32::sin(anim_time * 6.0);
    let radius = base_radius + pulse;

    draw_circle(center_x + 1.0, center_y + 1.0, radius, Color::new(0.0, 0.0, 0.0, 0.5));
    draw_circle(center_x, center_y, radius, RED);
    draw_circle_lines(center_x, center_y, radius, 1.5, Color::new(0.6, 0.0, 0.0, 1.0));

    let leaf_y = center_y - radius;
    draw_circle(center_x, leaf_y, 3.0, GREEN);
    draw_triangle(
        Vec2::new(center_x - 2.0, leaf_y - 1.0),
        Vec2::new(center_x + 2.0, leaf_y - 1.0),
        Vec2::new(center_x, leaf_y - 6.0),
        GREEN,
    );
}

pub fn draw_game_over(score: u32) {
    draw_rectangle(0.0, 0.0, 800.0, 600.0, Color::new(0.0, 0.0, 0.0, 0.7));
    draw_text("GAME OVER", 200.0, 200.0, 60.0, RED);
    draw_text(
        &format!("Your Score: {}", score),
        250.0,
        280.0,
        35.0,
        WHITE,
    );
    draw_text("Press Enter to Restart", 240.0, 350.0, 28.0, GRAY);
    draw_text("Press Escape for Menu", 250.0, 390.0, 28.0, GRAY);
}