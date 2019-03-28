use sdl2::pixels::Color;

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