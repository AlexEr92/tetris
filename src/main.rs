use sdl2::rect::Rect;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::thread::sleep;
use std::time::Duration;

mod game;

fn main() {
    // Initialize SDL context
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context
        .video()
        .expect("Couldn't get SDL video subsystem");

    // Create a window
    let window = video_subsystem
        .window("sdl2 demo", 800, 600)
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

    // Create a 32x32 texture to use as a square sprite
    let mut square_texture = texture_creator
        .create_texture_target(None, 32, 32)
        .expect("Failed to create texture");

    // Fill the texture with green color
    canvas
        .with_texture_canvas(&mut square_texture, |texture| {
            texture.set_draw_color(Color::RGB(0, 255, 0));
            texture.clear();
        })
        .expect("Failed to color a texture");

    // Main game loop flag and event pump
    let mut running = true;
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event pump");

    // Main game loop
    while running {
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
                _ => {}
            }
        }

        // Clear the screen with red background
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();

        // Draw the green square texture at the top-left corner
        canvas
            .copy(&square_texture, None, Rect::new(0, 0, 32, 32))
            .expect("Couldn't copy texture into window");

        // Present the rendered frame to the screen
        canvas.present();

        // Limit the frame rate to approximately 60 FPS
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
