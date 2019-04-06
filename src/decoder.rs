#![allow(non_snake_case)]
use crate::image::Image;

extern crate sdl2;
use sdl2::pixels::Color;

use std::io::BufReader;
use std::fs::File;

use std::f32::consts::PI;

use crate::primitives::*;
use crate::reader::data_reader;

fn cc(i: usize, j: usize) -> f32 {
    if i == 0 && j == 0 {
        return 1.0 / 2.0;
    } else if i == 0 || j == 0 {
        return 1.0 / (2.0 as f32).sqrt();
    } else {
        return 1.0;
    }
}

fn chomp(x: f32) -> u8 {
    if x >= 255.0 {
        return 255;
    } else if x <= 0.0 {
        return 0;
    } else {
        return x.round() as u8;
    }
}

const ZZ: [[usize; 8]; 8] = [
    [ 0,  1,  5,  6, 14, 15, 27, 28 ],
    [ 2,  4,  7, 13, 16, 26, 29, 42 ],
    [ 3,  8, 12, 17, 25, 30, 41, 43 ],
    [ 9, 11, 18, 24, 31, 40, 44, 53 ],
    [ 10, 19, 23, 32, 39, 45, 52, 54 ],
    [ 20, 22, 33, 38, 46, 51, 55, 60 ],
    [ 21, 34, 37, 47, 50, 56, 59, 61 ],
    [ 35, 36, 48, 49, 57, 58, 62, 63 ]
];

struct MCUWrap<'a> {
    mcu: MCU,
    jpeg_meta_data: &'a JPEGMetaData,
} 

impl<'a> MCUWrap<'a> {
    fn new( mcu: MCU, jpeg_meta_data: &'a JPEGMetaData) -> MCUWrap<'a> {
        return MCUWrap{ mcu, jpeg_meta_data };
    }
    fn display(&mut self, id: usize, h: usize, w: usize) {
        println!("mcu {} {} {}", id, h, w);
        let block = &self.mcu[id][h][w];
        for i in 0..8 {
            for j in 0..8 {
                print!("{} ", block[i][j]);
            }
            println!("");
        }
    }
    fn decode(&mut self) {
        let sof_info = &self.jpeg_meta_data.sof_info;
        let component_infos = &sof_info.component_infos;
        let quant_tables = &self.jpeg_meta_data.quant_tables;

        for id in 0..3 {
            let c_info = &component_infos[id];
            for h in 0..(c_info.vertical_sampling as usize) {
                for w in 0..(c_info.horizontal_sampling as usize) {

                    // println!("原始 mcu");
                    // self.display(id, h, w);
                    // 反量化
                    for i in 0..8 {
                        for j in 0..8 {
                            self.mcu[id][h][w][i][j] *= quant_tables[c_info.quant_table_id as usize][i*8 + j];
                        }
                    }
                    // println!("反量化之後");
                    // self.display(id, h, w);
                    // zigzag
                    let mut tmp: [[f32; 8]; 8] = Default::default();
                    for i in 0..8 {
                        for j in 0..8 {
                            tmp[i][j] = self.mcu[id][h][w][ZZ[i][j] / 8][ZZ[i][j] % 8];
                        }
                    }
                    self.mcu[id][h][w] = tmp;
                    // println!("zigzag 之後");
                    // self.display(id, h, w);
                    // 反向離散餘弦變換（idct）
                    tmp = Default::default();
                    for i in 0..8 {
                        for j in 0..8 {
                            for x in 0..8 {
                                for y in 0..8 {
                                    let i_cos = ((2*i+1) as f32 * PI / 16.0 * x as f32).cos();
                                    let j_cos =((2*j+1) as f32 * PI / 16.0 * y as f32).cos();
                                    tmp[i][j] += cc(x, y) * self.mcu[id][h][w][x][y] * i_cos * j_cos;
                                }
                            }
                            tmp[i][j] /= 4.0;
                        }
                    }
                    self.mcu[id][h][w] = tmp;
                    // println!("離散餘弦變換之後");
                    // self.display(id, h, w);
                }
            }
        }
    }
    fn toRGB(&mut self) -> Vec<Vec<Color>> {
        self.decode();

        let sof_info = &self.jpeg_meta_data.sof_info;
        let component_infos = &sof_info.component_infos;
        let max_vertical_sampling = sof_info.max_vertical_sampling;
        let max_horizontal_sampling = sof_info.max_horizontal_sampling;
        let mcu_height = 8 * max_vertical_sampling;
        let mcu_width = 8 * max_horizontal_sampling;

        let mut ret = vec![vec![Color::RGB(0, 0, 0); mcu_width as usize]; mcu_height as usize];
        for i in 0..mcu_height {
            for j in 0..mcu_width {
                // 獲取 Y, Cb, Cr 三個顏色分量所對應的採樣
                let mut YCbCr = [0.0; 3];
                for id in 0..3 {
                    let vh = (i * component_infos[id].vertical_sampling / max_vertical_sampling) as usize;
                    let vw = (j * component_infos[id].horizontal_sampling / max_horizontal_sampling) as usize;
                    YCbCr[id] = self.mcu[id][vh / 8][vw / 8][vh % 8][vw % 8];
                }
                let (Y, Cb, Cr) = (YCbCr[0], YCbCr[1], YCbCr[2]);
                // let (Y, Cb, Cr) = (YCbCr[0], 0.0, 0.0);
                let R = chomp(Y + 1.402*Cr + 128.0);
                let G = chomp(Y - 0.34414*Cb - 0.71414*Cr + 128.0);
                let B = chomp(Y + 1.772*Cb + 128.0);
                ret[i as usize][j as usize] = Color::RGB(R, G, B);
            }
        }
        return ret;
    }
}

pub fn decoder(reader: BufReader<File>) -> Image {
    let (jpeg_meta_data, MCUs) = data_reader(reader);

    let sof_info = &jpeg_meta_data.sof_info;
    let mcu_width = 8 * sof_info.max_horizontal_sampling as usize;
    let mcu_height = 8 * sof_info.max_vertical_sampling as usize;

    // 寬度上有幾個 MCU
    let mcu_width_number = ((sof_info.width as usize - 1) / mcu_width + 1) as usize;
    // 高度上有幾個 MCU
    let mcu_height_number = ((sof_info.height as usize - 1) / mcu_height + 1) as usize;

    let image_width = (mcu_width_number * mcu_width) as u32;
    let image_height = (mcu_height_number * mcu_height) as u32;
    let mut image = Image::new(image_width, image_height);

    for h in 0..mcu_height_number {
        for w in 0..mcu_width_number {
            let mcu = MCUs[h][w].clone();
            let mcu_rgb = MCUWrap::new(mcu, &jpeg_meta_data).toRGB();
            for y in 0..mcu_height {
                for x in 0..mcu_width {
                    image.pixels[h*mcu_height + y][w*mcu_width + x] = mcu_rgb[y][x];
                }
            }
        }
    }

    // MCUWrap::new(MCUs[20][20].clone(), &jpeg_meta_data).toRGB();
    return image;
}