use crate::color::Color;
use crate::framebuffer::Framebuffer;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub body: Vec<(usize, usize)>,
    direction: Direction,
    grow: bool,
    color: Color,
}

impl Snake {
    pub fn new(start_x: usize, start_y: usize, color: Color) -> Self {
        let body = vec![
            (start_x, start_y),     // Head
            (start_x - 1, start_y), // Middle segment
            (start_x - 2, start_y), // Tail segment
        ];
        Self {
            body,
            direction: Direction::Right,
            grow: false,
            color,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn move_forward(&mut self) {
        let (head_x, head_y) = self.body[0];
        let new_head = match self.direction {
            Direction::Up => (head_x, head_y.wrapping_sub(1)),
            Direction::Down => (head_x, head_y.wrapping_add(1)),
            Direction::Left => (head_x.wrapping_sub(1), head_y),
            Direction::Right => (head_x.wrapping_add(1), head_y),
        };

        self.body.insert(0, new_head);
        if !self.grow {
            self.body.pop();
        } else {
            self.grow = false;
        }
    }

    pub fn grow(&mut self) {
        self.grow = true;
    }

    pub fn head_position(&self) -> (usize, usize) {
        self.body[0]
    }

    pub fn check_collision(&self) -> bool {
        let head = self.head_position();
        self.body.iter().skip(1).any(|&pos| pos == head)
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer, grid_size: usize) {
        for &(x, y) in &self.body {
            framebuffer.draw_rectangle(
                x * grid_size,
                y * grid_size,
                grid_size,
                grid_size,
                self.color,
            );
        }
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }
}
