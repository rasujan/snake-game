use piston_window::types::Color;
use piston_window::*;

use crate::draw::{draw_block, draw_rectangle};
use rand::{thread_rng, Rng};

const BORDER_COLOR: Color = [0.252, 0.202, 0.70, 1.0];
const FOOD_COLOR: Color = [0.0, 0.6, 0.0, 1.0];

pub struct Game {
    //Food
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    //Border
    width: i32,
    height: i32,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            //food
            food_exists: false,
            food_x: 0,
            food_y: 0,
            //Border
            width,
            height,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // Draw food
        if (self.food_exists) {
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

    pub fn add_food(&mut self) {
        let mut rag = thread_rng();
        let new_x = rag.gen_range(1..self.width - 1);
        let new_y = rag.gen_range(1..self.height - 1);

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    pub fn update(&mut self, delta_time: f64) {
        if !self.food_exists {
            self.add_food()
        }
    }
}
