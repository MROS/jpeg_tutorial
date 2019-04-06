#![allow(non_snake_case)]
use crate::image::Image;

use std::io::BufReader;
use std::fs::File;

pub fn decoder(mut reader: BufReader<File>) -> Image {
    return Image::new(800, 600);
}