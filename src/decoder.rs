use crate::image::Image;

use std::io::BufReader;
use std::fs::File;
use std::io::Read;

enum Marker {
    SOI = 0xD8,      
    APP0 = 0xE0,
    DQT = 0xDB,
    SOF = 0xC0,
    DHT = 0xC4,
    SOS = 0xDA,
    EOI = 0xD9,      
    COM = 0xFE,
}

const MARKER_PREFIX: u8 = 0xFF;

const SOI_MARKER: u8 = 0xD8;      // start of image: 圖片起始
const EOI_MARKER: u8 = 0xD9;      // end of image: 圖片結束

const APP0_MARKER: u8 = 0xE0;

const DQT_MARKER: u8 = 0xDB;
const DHT_MARKER: u8 = 0xC4;

const SOF_MARKER: u8 = 0xC0;
const SOS_MARKER: u8 = 0xDA;

const COM_MARKER: u8 = 0xFE;

pub fn decoder(mut reader: BufReader<File>) -> Image {
    let mut c: [u8; 1] = [0; 1];

    loop {
        reader.read(&mut c);
        if c[0] != MARKER_PREFIX {
            continue;
        }

        reader.read(&mut c);
        match c[0] {
            0xD8 => {
                println!("掃過 SOI marker ，圖片起始");
            },
            0xD9 => {
                println!("掃過 EOI marker ，圖片結束");
                break;
            },
            0x00 => {

            }
            m => {
                println!("other marker: {:#X?}", m);
            }
        }

    }

    return Image::new(800, 600);
}