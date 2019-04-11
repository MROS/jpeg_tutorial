#![allow(non_snake_case)]

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn RGB(r: u8, g: u8, b: u8) -> Color {
        return Color {r, g, b};
    }
}

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Vec<Color>>
}

impl Image {
    // 初始化爲全黑的
    pub fn new(width: u32, height: u32) -> Image {
        return Image{
            width: width,
            height: height,
            pixels: vec![vec![Color::RGB(0, 0, 0); width as usize]; height as usize]
        };
    }
}