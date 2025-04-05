use sdl2::rect::Rect;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let sdl_context = sdl2::init().expect("SDL initialization failed");
    let video_subsystem = sdl_context
        .video()
        .expect("Couldn't get SDL video subsystem");

    let window = video_subsystem
        .window("sdl2 demo", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .expect("Failed to create window");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Failed to convert window into canvas");

    let texture_creator = canvas.texture_creator();

    let mut square_texture = texture_creator
        .create_texture_target(None, 32, 32)
        .expect("Failed to create texture");

    canvas
        .with_texture_canvas(&mut square_texture, |texture| {
            texture.set_draw_color(Color::RGB(0, 255, 0));
            texture.clear();
        })
        .expect("Failed to color a texture");

    let mut running = true;
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Failed to get SDL event pump");

    while running {
        for event in event_pump.poll_iter() {
            match event {
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
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.clear();
        canvas
            .copy(&square_texture, None, Rect::new(0, 0, 32, 32))
            .expect("Couldn't copy texture into window");
        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
