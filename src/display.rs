use crate::image::Image;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

use std::fs::File;
use std::io::prelude::*;

pub fn to_ppm(image: Image) -> std::io::Result<()> {
    let mut file = File::create("out.ppm")?;

    write!(file, "P6\n{} {}\n255\n", image.width, image.height)?;
    for row in 0..image.height {
        for col in 0..image.width {
            let pixel = image.pixels[row as usize][col as usize];
            file.write(&[pixel.r, pixel.g, pixel.b])?;
        }
    }
    Ok(())
}

pub fn display_image(image: Image) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("image", image.width, image.height)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();

    for row in 0..image.height {
        for col in 0..image.width {
            canvas.set_draw_color(image.pixels[row as usize][col as usize]);
            let result = canvas.draw_point(Point::new(col as i32, row as i32));
            match result {
                Ok(_) => {},
                Err(e) => println!("draw_point error: {:?}", e),
            }
        }
    }
    canvas.present();

    // 按 Esc 退出
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
    }
}
