use crate::image::Image;

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