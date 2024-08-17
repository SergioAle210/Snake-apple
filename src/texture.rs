extern crate image;

use crate::color::Color;
use image::{DynamicImage, GenericImageView};
use image::{ImageReader, RgbImage};

pub struct Texture {
    image: RgbImage,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(file_path: &str) -> Texture {
        let img = ImageReader::open(file_path)
            .unwrap()
            .decode()
            .unwrap()
            .to_rgb8();
        let width = img.width();
        let height = img.height();
        Texture {
            image: img,
            width,
            height,
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let pixel = self.image.get_pixel(x as u32, y as u32);
        Color::new(pixel[0] as i32, pixel[1] as i32, pixel[2] as i32)
    }
}
