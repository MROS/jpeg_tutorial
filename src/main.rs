mod image;
mod display;
mod decoder;

use image::Image;
use display::display_image;
use decoder::decoder;

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

    let mut f = File::open(filename)?;
    let mut buffer: Vec<u8> = Vec::new();

    f.read_to_end(&mut buffer)?;

    let image: Image = decoder(buffer);

    display_image(image);
    Ok(())
}