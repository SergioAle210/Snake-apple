use crate::color::Color;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![Color::new(0, 0, 0); width * height]; // Black background
        Self {
            buffer,
            width,
            height,
            background_color: Color::new(0, 0, 0), // Black background
            current_color: Color::new(255, 255, 255), // Default drawing color is white
        }
    }

    pub fn point_with_color(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn set_background_color(&mut self, color: impl Into<Color>) {
        self.background_color = color.into();
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = Color::from_hex(color);
    }

    pub fn draw_rectangle(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: Color,
    ) {
        for i in 0..width {
            for j in 0..height {
                self.point_with_color(x + i, y + j, color.clone());
            }
        }
    }

    pub fn clear(&mut self) {
        for pixel in &mut self.buffer {
            *pixel = self.background_color.clone(); // Clear to background color
        }
    }

    pub fn is_point_set(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] == self.current_color
        } else {
            false
        }
    }

    pub fn to_u32_buffer(&self) -> Vec<u32> {
        self.buffer.iter().map(|color| color.to_hex()).collect()
    }
}
