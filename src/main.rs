extern crate sdl2;


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


mod gfx {
    pub fn hello() {
        println!("wassup")
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    gfx::hello();

    let window = video_subsystem.window("Hello, SDL2!", 640, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    
    // change the color of our drawing with a gold-color ...
    canvas.clear();
     // A draw a rectangle which almost fills our window with it !
    canvas.set_draw_color(Color::RGB(25, 210, 0));
    canvas.fill_rect(sdl2::rect::Rect::new(10, 10, 640/2,480/2));

    canvas.present(); 
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
    }
}

