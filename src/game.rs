use std::hint::select_unpredictable;
use  crate::game_menu_action::*;

use crate::snake::Snake;
use crate::food::random_food;
use crate::render;
use crate::menu::*;
use crate::game_state::GameState;
use macroquad::prelude::*;


const MOVE_SPEED: f32 = 0.12;



pub struct Game {
    snake: Snake,
    food: crate::point::Point,
    state: GameState,
    score: u32,
    move_timer: f32,
    anim_time: f32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            snake: Snake::new(),
            food: random_food(),
            state: GameState::Menu,
            score: 0,
            move_timer: 0.0,
            anim_time: 0.0,
        }
    }

    fn start_new_game(&mut self) {
        self.snake = Snake::new();
        self.food = random_food();
        self.score = 0;
        self.move_timer = 0.0;
        self.state = GameState::Playing;
    }

    fn open_settion(&mut self)
    {
        self.state = GameState::Setting;
    }

    pub fn update(&mut self) {
        self.anim_time += get_frame_time();

        match self.state {
            GameState::Menu => {
                match Menu::draw() {
                    GameMenuAction::Start => self.start_new_game(),
                    GameMenuAction::Quit => std::process::exit(0),
                    GameMenuAction::None => {}
                }
            }
            GameState::Playing => {
                if is_key_pressed(KeyCode::Up) && self.snake.dir.y == 0 {
                    self.snake.dir = crate::point::Point { x: 0, y: -1 };
                }
                if is_key_pressed(KeyCode::Down) && self.snake.dir.y == 0 {
                    self.snake.dir = crate::point::Point { x: 0, y: 1 };
                }
                if is_key_pressed(KeyCode::Left) && self.snake.dir.x == 0 {
                    self.snake.dir = crate::point::Point { x: -1, y: 0 };
                }
                if is_key_pressed(KeyCode::Right) && self.snake.dir.x == 0 {
                    self.snake.dir = crate::point::Point { x: 1, y: 0 };
                }

                self.move_timer += get_frame_time();
                if self.move_timer >= MOVE_SPEED {
                    self.move_timer = 0.0;
                    if !self.snake.update(&mut self.food) {
                        self.state = GameState::GameOver;
                    } else {
                        self.score = self.snake.len() as u32 - 3;
                    }
                }

                if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Menu;
                }
            }
            GameState::Setting => {
                if is_key_pressed(KeyCode::Escape)
                {
                    self.state = GameState::Menu;
                }
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Enter) {
                    self.start_new_game();
                }
                if is_key_pressed(KeyCode::Escape) {
                    self.state = GameState::Menu;
                }
            }
        }
    }

    pub fn draw(&self) {
        match self.state {
            GameState::Menu => {
                Menu::draw();
            }
            GameState::Playing => {
                render::draw_grid();
                render::draw_food(&self.food, self.anim_time);
                render::draw_snake(&self.snake);
                draw_text(
                    &format!("Score: {}", self.score),
                    render::OFFSET_X,
                    render::OFFSET_Y - 15.0,
                    28.0,
                    WHITE,
                );
            }
            GameState::Setting => {
                
            }
            GameState::GameOver => {
                render::draw_grid();
                render::draw_food(&self.food, self.anim_time);
                render::draw_snake(&self.snake);
                draw_text(
                    &format!("Score: {}", self.score),
                    render::OFFSET_X,
                    render::OFFSET_Y - 15.0,
                    28.0,
                    WHITE,
                );
                render::draw_game_over(self.score);
            }
        }
    }
}