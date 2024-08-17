mod color;
mod framebuffer;
mod snake;

use color::Color;
use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use snake::{Direction, Snake};
use std::time::{Duration, Instant};

fn main() {
    let width = 1300;
    let height = 900;
    let grid_size = 20; // Size of each grid cell
    let mut framebuffer = Framebuffer::new(width, height);

    framebuffer.set_background_color(Color::new(0, 0, 0)); // Set to black

    let mut window = Window::new(
        "Snake - Apple",
        (width as f32 / 1.3) as usize,
        (height as f32 / 1.3) as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut snake = Snake::new(
        width / (2 * grid_size),
        height / (2 * grid_size),
        Color::new(44, 86, 176),
    );
    let mut apple = spawn_apple(&snake, width / grid_size, height / grid_size);

    let mut last_update = Instant::now();
    let update_interval = Duration::from_millis(100);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if last_update.elapsed() >= update_interval {
            framebuffer.clear(); // Clear to background color

            // Handle input
            if window.is_key_down(Key::Up) {
                snake.set_direction(Direction::Up);
            } else if window.is_key_down(Key::Down) {
                snake.set_direction(Direction::Down);
            } else if window.is_key_down(Key::Left) {
                snake.set_direction(Direction::Left);
            } else if window.is_key_down(Key::Right) {
                snake.set_direction(Direction::Right);
            }

            // Move the snake
            snake.move_forward();

            // Check for collisions
            let (head_x, head_y) = snake.head_position();
            if head_x >= width / grid_size
                || head_y >= height / grid_size
                || snake.check_collision()
            {
                println!("Game Over!");
                break;
            }

            // Check if the snake eats the apple
            if snake.head_position() == apple {
                snake.grow();
                apple = spawn_apple(&snake, width / grid_size, height / grid_size);
            }

            // Draw the apple
            framebuffer.draw_rectangle(
                apple.0 * grid_size,
                apple.1 * grid_size,
                grid_size,
                grid_size,
                Color::new(255, 0, 0),
            );

            // Draw the snake
            snake.draw(&mut framebuffer, grid_size);

            // Update the window
            window
                .update_with_buffer(&framebuffer.to_u32_buffer(), width, height)
                .unwrap();

            last_update = Instant::now();
        }

        window.update();
    }
}

fn spawn_apple(snake: &Snake, width: usize, height: usize) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    loop {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        if !snake.body.contains(&(x, y)) {
            return (x, y);
        }
    }
}
