mod color;
mod framebuffer;
mod snake;

use color::Color;
use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use rodio::{source::Source, Decoder, OutputStream, Sink};
use snake::{Direction, Snake};
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const HIGH_SCORE_FILE: &str = "high_score.txt";

fn play_background_music(sink: Arc<Mutex<Sink>>, stream_handle: rodio::OutputStreamHandle) {
    let file = File::open("./assets/study.mp3").expect("Failed to open music file");
    let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio");

    let amplified_source = source.amplify(0.3);

    let mut sink = sink.lock().unwrap();
    sink.append(amplified_source.repeat_infinite());
    sink.play();
}

fn load_high_score() -> u32 {
    if let Ok(contents) = fs::read_to_string(HIGH_SCORE_FILE) {
        if let Ok(score) = contents.trim().parse() {
            return score;
        }
    }
    0
}

fn save_high_score(score: u32) {
    if let Ok(mut file) = File::create(HIGH_SCORE_FILE) {
        let _ = writeln!(file, "{}", score);
    }
}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let bg_music_sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));

    // Start playing background music in a separate thread.
    let bg_music_sink_clone = Arc::clone(&bg_music_sink);
    let stream_handle_clone = stream_handle.clone();
    thread::spawn(move || {
        play_background_music(bg_music_sink_clone, stream_handle_clone);
    });

    let width = 1300;
    let height = 900;
    let grid_size = 40; // Size of each grid cell
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

    let mut high_score = load_high_score();
    let mut current_score = 0;

    loop {
        let mut snake = Snake::new(
            width / (2 * grid_size),
            height / (2 * grid_size),
            Color::new(44, 86, 176),
        );
        let mut apple = spawn_apple(&snake, width / grid_size, height / grid_size);

        let mut last_update = Instant::now();
        let update_interval = Duration::from_millis(100);

        let mut game_over = false;

        while window.is_open() && !game_over {
            if window.is_key_down(Key::Escape) {
                return; // Exit the game
            }

            if last_update.elapsed() >= update_interval {
                framebuffer.clear(); // Clear to background color

                // Draw the border and grid pattern
                draw_border_and_grid(&mut framebuffer, width, height, grid_size);

                // Handle input with direction validation
                if window.is_key_down(Key::Up) && snake.direction() != Direction::Down {
                    snake.set_direction(Direction::Up);
                } else if window.is_key_down(Key::Down) && snake.direction() != Direction::Up {
                    snake.set_direction(Direction::Down);
                } else if window.is_key_down(Key::Left) && snake.direction() != Direction::Right {
                    snake.set_direction(Direction::Left);
                } else if window.is_key_down(Key::Right) && snake.direction() != Direction::Left {
                    snake.set_direction(Direction::Right);
                }

                // Move the snake
                snake.move_forward();

                // Check for collisions
                let (head_x, head_y) = snake.head_position();
                if head_x == 0
                    || head_x == width / grid_size - 1
                    || head_y == 0
                    || head_y == height / grid_size - 1
                    || snake.check_collision()
                {
                    game_over = true;
                    break;
                }

                // Check if the snake eats the apple
                if snake.head_position() == apple {
                    snake.grow();
                    current_score += 1;
                    if current_score > high_score {
                        high_score = current_score;
                        save_high_score(high_score);
                    }
                    apple = spawn_apple(&snake, width / grid_size, height / grid_size);
                }

                // Draw the apple
                framebuffer.draw_rectangle(
                    apple.0 * grid_size,
                    apple.1 * grid_size,
                    grid_size,
                    grid_size,
                    Color::new(129, 45, 214), // Grape color
                );

                // Draw the snake
                snake.draw(&mut framebuffer, grid_size);

                // Display the current score and high score
                display_scores(&mut framebuffer, current_score, high_score);

                // Update the window
                window
                    .update_with_buffer(&framebuffer.to_u32_buffer(), width, height)
                    .unwrap();

                last_update = Instant::now();
            }

            window.update();
        }

        // Game over logic
        if game_over {
            if current_score > high_score {
                high_score = current_score;
                save_high_score(high_score);
            }

            while window.is_open() {
                if window.is_key_down(Key::Escape) {
                    return; // Exit the game
                }
                if window.is_key_down(Key::Enter) || window.is_key_down(Key::Space) {
                    break; // Restart the game
                }
                window.update();
            }

            current_score = 0; // Reset the current score for the new game
        }
    }
}

fn draw_border_and_grid(
    framebuffer: &mut Framebuffer,
    width: usize,
    height: usize,
    grid_size: usize,
) {
    let border_color = Color::new(144, 12, 63);
    let color1 = Color::new(230, 176, 170);
    let color2 = Color::new(215, 189, 226);

    // Draw the border
    for x in 0..width / grid_size {
        framebuffer.draw_rectangle(x * grid_size, 0, grid_size, grid_size, border_color);
        framebuffer.draw_rectangle(
            x * grid_size,
            (height / grid_size - 1) * grid_size,
            grid_size,
            grid_size,
            border_color,
        );
    }

    for y in 0..height / grid_size {
        framebuffer.draw_rectangle(0, y * grid_size, grid_size, grid_size, border_color);
        framebuffer.draw_rectangle(
            (width / grid_size - 1) * grid_size,
            y * grid_size,
            grid_size,
            grid_size,
            border_color,
        );
    }

    // Draw the grid pattern inside the border
    for y in 1..height / grid_size - 1 {
        for x in 1..width / grid_size - 1 {
            let color = if (x + y) % 2 == 0 { color1 } else { color2 };
            framebuffer.draw_rectangle(x * grid_size, y * grid_size, grid_size, grid_size, color);
        }
    }
}

fn display_scores(framebuffer: &mut Framebuffer, current_score: u32, high_score: u32) {
    let score_text = format!("Score: {}", current_score);
    let high_score_text = format!("High Score: {}", high_score);

    // Draw current score in the top left corner
    framebuffer.draw_text(&score_text, 10, 10, Color::new(255, 255, 255)); // White color

    // Draw high score in the top right corner
    let high_score_x = framebuffer.width - 200; // Adjust based on text width
    framebuffer.draw_text(
        &high_score_text,
        high_score_x,
        10,
        Color::new(255, 255, 255),
    ); // White color
}

fn spawn_apple(snake: &Snake, width: usize, height: usize) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    loop {
        let x = rng.gen_range(1..width - 1); // Avoid spawning on the border
        let y = rng.gen_range(1..height - 1); // Avoid spawning on the border
        if !snake.body.contains(&(x, y)) {
            return (x, y);
        }
    }
}
