use crate::draw::draw_block;
use piston_window::types::Color;
use piston_window::{Context, G2d};

use std::collection::LinkedList;
use std::collections::LinkedList;

const SNAKE_COLOR: Color = [0.08, 0.0, 0.0, 0.9];

#[Derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[Derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn snake(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();

        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Snake {
            direction: Direction::Right,
            body,
            tail: none,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g)
        }
    }

    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(dir) => self.direction = d,
            None => {}
        }
        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block: Block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
        };

        self.body.push_front(new_block);
        let removed_block: block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();
        let mut moving_direction = self.direction;

        match dir {
            Some(dir) => moving_direction = dir,
            None => {}
        }

        match moving_direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Right => (head_x + 1, head_y),
            Direction::Left => (head_x - 1, head_y),
        }
    }

    pub fn restore_tail(&mut self) {
        let _block = self.tail.clone().unwrap();
        self.body.push_back(_block)
    }
}