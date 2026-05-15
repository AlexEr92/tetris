use std::time::{Duration, Instant};

use sdl2::rect::Rect;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::thread::sleep;

mod game;
use game::Tetris;

fn main() {
    // Initialize SDL context
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context
        .video()
        .expect("Couldn't get SDL video subsystem");

    // Create a window
    let window = video_subsystem
        .window("Tetris", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    // Create a canvas to draw on
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to convert window into canvas");

    // Create a texture creator to create textures
    let texture_creator = canvas.texture_creator();

    // Create 32x32 textures for each piece color (1..=7)
    let mut square_textures = Vec::new();
    let colors = [
        Color::RGB(0, 255, 255), // 1: I - cyan
        Color::RGB(255, 165, 0), // 2: L - orange
        Color::RGB(0, 0, 255),   // 3: J - blue
        Color::RGB(255, 255, 0), // 4: O - yellow
        Color::RGB(0, 255, 0),   // 5: S - green
        Color::RGB(255, 0, 0),   // 6: Z - red
        Color::RGB(160, 0, 200), // 7: T - purple
    ];
    for color in colors {
        let mut texture = texture_creator
            .create_texture_target(None, 32, 32)
            .expect("Failed to create texture");
        canvas
            .with_texture_canvas(&mut texture, |texture| {
                texture.set_draw_color(color);
                texture.clear();
            })
            .expect("Failed to color a texture");
        square_textures.push(texture);
    }

    let mut game = Tetris::new();

    // Main game loop flag and event pump
    let mut running = true;
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event pump");

    // Timers
    let mut last_drop = Instant::now();
    let mut last_frame = Instant::now();
    let target_frame_time = Duration::from_nanos(1_000_000_000u64 / 60);

    // Field position on screen (centered in 800x600)
    let field_x: i32 = 240;
    let field_y: i32 = 50;

    // Main game loop
    while running && !game.game_over {
        let frame_start = Instant::now();
        let delta = frame_start - last_frame;
        last_frame = frame_start;

        // Handle window and keyboard events
        for event in event_pump.poll_iter() {
            match event {
                // Exit on window close or Escape key press
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    game.move_left();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    game.move_right();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    game.rotate();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    game.soft_drop();
                }
                _ => {}
            }
        }

        // Gravity: auto-drop based on level
        if last_drop.elapsed() >= game.drop_interval() {
            game.move_down();
            last_drop = Instant::now();
        }

        // Lock delay countdown
        if game.tick_lock_delay(delta) {
            game.lock_tetromino();
            game.spawn_tetromino();
            last_drop = Instant::now();
        }

        // Render
        canvas.set_draw_color(Color::RGB(20, 20, 20));
        canvas.clear();

        // Draw field background
        canvas.set_draw_color(Color::RGB(40, 40, 40));
        let _ = canvas.fill_rect(Rect::new(field_x, field_y, 320, 512));

        // Draw locked pieces (game_map)
        let game_map = game.game_map();
        for (y, row) in game_map.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell > 0 && cell <= 7 {
                    let texture_idx = (cell - 1) as usize;
                    let rect = Rect::new(field_x + x as i32 * 32, field_y + y as i32 * 32, 32, 32);
                    let _ = canvas.copy(&square_textures[texture_idx], None, rect);
                }
            }
        }

        // Draw active tetromino
        if let Some(ref tetromino) = *game.current_tetromino() {
            let state = &tetromino.states[tetromino.current_state as usize];
            for (dy, row) in state.iter().enumerate() {
                for (dx, &cell) in row.iter().enumerate() {
                    if cell > 0 && cell <= 7 {
                        let texture_idx = (cell - 1) as usize;
                        let rect = Rect::new(
                            field_x + (tetromino.x + dx as isize) as i32 * 32,
                            field_y + (tetromino.y + dy) as i32 * 32,
                            32,
                            32,
                        );
                        let _ = canvas.copy(&square_textures[texture_idx], None, rect);
                    }
                }
            }
        }

        canvas.present();

        // Limit the frame rate to approximately 60 FPS
        let elapsed = frame_start.elapsed();
        if elapsed < target_frame_time {
            sleep(target_frame_time - elapsed);
        }
    }
}
