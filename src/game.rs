use piston_window::types::Color;
use piston_window::*;
use rand::{thread_rng, Rng};

use crate::draw::{draw_circle, draw_rectangle};
use crate::snake::{Direction, Snake};

const BORDER_COLOR: Color = [0.4, 0.31, 0.37, 0.1]; // Very Dark Green
const FOOD_COLOR: Color = [0.0, 0.6, 0.0, 1.0]; // Light Green
const GAME_OVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5]; // Transparent Red;

const MOVING_PERIOD: f64 = 0.2;
const RESTART_TIME: f64 = 1.0;

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
    game_over: bool,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            // Food
            food_exists: false,
            food_x: 0,
            food_y: 0,
            // Border
            width,
            height,
            // Misc
            waiting_time: 0.0,
            game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

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
            draw_circle(FOOD_COLOR, self.food_x, self.food_y, con, g);
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

        // Game Over
        if self.game_over {
            draw_rectangle(GAME_OVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
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
        let mut new_x = rag.gen_range(1..self.width - 1);
        let mut new_y = rag.gen_range(1..self.height - 1);

        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rag.gen_range(1..self.width - 1);
            new_y = rag.gen_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart()
            }
            return;
        }

        if !self.food_exists {
            self.add_food()
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None)
        }
    }

    fn check_if_snake_is_alive(&self, direction: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(direction);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        // Checking if the snake is inside the box
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn update_snake(&mut self, direction: Option<Direction>) {
        if self.check_if_snake_is_alive(direction) {
            self.snake.move_forward(direction);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = false;
        self.food_x = 0;
        self.food_y = 0;
        self.game_over = false;
    }
}
