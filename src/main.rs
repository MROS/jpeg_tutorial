mod image;
mod display;

use image::Image;
use display::display_image;

use sdl2::pixels::Color;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let length = args.len();
    if length != 2 {
        println!("用法：cargo run XXX.jpg");
        return Ok(());
    }
    let filename = &args[1];
    println!("嘗試解碼 {}", filename);

    let mut f = File::open(filename)?;
    let mut buffer = [0; 10];

    f.read(&mut buffer)?;
    println!("{:?}", buffer);

    let mut image = Image::new(500, 50);
    for row in 0..image.height {
        for col in 0..image.width {
            let gray: u8 = (col * 255 / image.width) as u8;
            image.pixels[row as usize][col as usize] = Color::RGB(gray, gray, gray);
        }
    }
    display_image(image);
    Ok(())
}