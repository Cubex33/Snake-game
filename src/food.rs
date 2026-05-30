use crate::point::Point;
use ::rand::{thread_rng, Rng};

pub const GRID_WIDTH: i32 = 20;
pub const GRID_HEIGHT: i32 = 20;

pub fn random_food() -> Point {
    let mut rng = thread_rng();
    Point {
        x: rng.gen_range(0..GRID_WIDTH),
        y: rng.gen_range(0..GRID_HEIGHT),
    }
}
