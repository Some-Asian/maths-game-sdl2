use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;

mod q_gen;
use q_gen::Equation;

fn render(canvas: &mut WindowCanvas, color: Color) {
    canvas.set_draw_color(color);
    canvas.draw_rect(Rect::new(10, 10, 50, 90));
    canvas.present();
}
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Minesweeper (soon enough)", 600, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut r: u8 = 0;
    let mut g: u8 = 0;
    let mut b: u8 = 0;
    'running: loop {
        // Events
        break 'running
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    r = (r + 1) % 255
                }
                Event::KeyDown { keycode: Some(Keycode::G), .. } => {
                    g = (g + 1) % 255
                }
                Event::KeyDown { keycode: Some(Keycode::B), .. } => {
                    b = (b + 1) % 255
                }
                _ => {}
            }
        };
        // Update

        // Render
        render(&mut canvas, Color {
            r, g, b, a: 255
        });

        canvas.present();

        // 60FPS
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    let mut t: Vec<Equation> = vec![];
    for i in 1..=100 {
        t.push(Equation::gen_equation(&mut rand::thread_rng(), Some(5)))
    }
    for w in t {
        w.print()
    }
}