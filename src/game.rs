use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

const BORDER_COLOR: Color = [0.252, 0.202, 0.70, 1.0];
const FOOD_COLOR: Color = [0.0, 0.6, 0.0, 1.0];

const MOVING_PERIOD: f64 = 0.2;

pub struct Game {
    // Snake
    snake: Snake,
    // Food
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    // Border
    width: i32,
    height: i32,
    // Misc
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            //food
            food_exists: false,
            food_x: 0,
            food_y: 0,
            //Border
            width,
            height,
            //misc
            waiting_time: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        let direction = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Right => Some(Direction::Right),
            Key::Left => Some(Direction::Left),
            _ => Some(self.snake.head_direction()),
        };

        // * can't go in opposite direction
        if direction.unwrap() == self.snake.head_direction().opposite() {
            return;
        };

        self.update_snake(direction);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // Draw Snake
        self.snake.draw(con, g);
        // Draw food
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }
        // This draws the border
        // Top
        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        // bottom
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        // Left
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        // Right
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn add_food(&mut self) {
        let mut rag = thread_rng();
        let new_x = rag.gen_range(1..self.width - 1);
        let new_y = rag.gen_range(1..self.height - 1);

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if !self.food_exists {
            self.add_food()
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None)
        }
    }

    fn update_snake(&mut self, direction: Option<Direction>) {
        self.snake.move_forward(direction);
        self.check_eating();
        self.waiting_time = 0.0;
    }
}
