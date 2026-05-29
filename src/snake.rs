use crate::point::Point;
use crate::food::{random_food, GRID_WIDTH, GRID_HEIGHT};

pub struct Snake {
    pub body: Vec<Point>,
    pub dir: Point,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![
                Point { x: 10, y: 10 },
                Point { x: 9, y: 10 },
                Point { x: 8, y: 10 },
            ],
            dir: Point { x: 1, y: 0 },
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