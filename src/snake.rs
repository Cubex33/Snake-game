use crate::point::Point;
use crate::food::{random_food, GRID_WIDTH, GRID_HEIGHT};
use macroquad::prelude::*;
use crate::data::*;

pub struct Snake {
    pub body: Vec<Point>,
    pub dir: Point,
    pub color: Color,
}

fn color_from_string(name: &str) -> Color {
    match name {
        "Red" => RED,
        "Blue" => BLUE,
        "Yellow" => YELLOW,
        "Purple" => PURPLE,
        "Orange" => ORANGE,
        _ => GREEN,
    }
}

impl Snake {
    pub fn new() -> Self {
        let data = GameData::load();

        Self {
            body: vec![
                Point { x: 5, y: 5 },
                Point { x: 4, y: 5 },
                Point { x: 3, y: 5 },
            ],
            dir: Point { x: 1, y: 0 },
            color: color_from_string(&data.color),
        }
    }

    pub fn update(&mut self, food: &mut Point) -> bool {
        let mut new_head = self.body[0];
        new_head.x += self.dir.x;
        new_head.y += self.dir.y;

        if new_head.x < 0 || new_head.y < 0
            || new_head.x >= GRID_WIDTH || new_head.y >= GRID_HEIGHT
        {
            return false;
        }

        if self.body.contains(&new_head) {
            return false;
        }

        self.body.insert(0, new_head);

        if new_head == *food {
            *food = random_food();
        } else {
            self.body.pop();
        }
        true
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }
}           