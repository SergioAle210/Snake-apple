use crate::color::Color;
use rusttype::{point, Font, PositionedGlyph, Scale};

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

    pub fn draw_text(&mut self, text: &str, x: usize, y: usize, color: Color) {
        // Cargar una fuente desde los datos incrustados (por ejemplo, OpenSans)
        let font_data = include_bytes!("../SIXTY.TTF");
        let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

        let scale = Scale { x: 40.0, y: 40.0 };

        let v_metrics = font.v_metrics(scale);

        let offset = point(0.0, v_metrics.ascent);

        let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, offset).collect();

        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|gx, gy, v| {
                    let px = x as i32 + gx as i32 + bb.min.x;
                    let py = y as i32 + gy as i32 + bb.min.y;
                    if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
                        let index = (py as usize) * self.width + (px as usize);
                        let alpha = (v * 255.0) as u32;
                        let existing_color = self.buffer[index].to_hex();
                        let new_color = blend_colors(existing_color, color.to_hex(), alpha);
                        self.buffer[index] = new_color.into();
                    }
                });
            }
        }
    }
}

fn blend_colors(existing: u32, new: u32, alpha: u32) -> u32 {
    let existing_r = (existing >> 16) & 0xFF;
    let existing_g = (existing >> 8) & 0xFF;
    let existing_b = existing & 0xFF;

    let new_r = (new >> 16) & 0xFF;
    let new_g = (new >> 8) & 0xFF;
    let new_b = new & 0xFF;

    let blended_r = (existing_r * (255 - alpha) + new_r * alpha) / 255;
    let blended_g = (existing_g * (255 - alpha) + new_g * alpha) / 255;
    let blended_b = (existing_b * (255 - alpha) + new_b * alpha) / 255;

    (blended_r << 16) | (blended_g << 8) | blended_b
}
