use crate::image::Image;

use std::io::BufReader;
use std::fs::File;
use std::io::Read;

fn marker_info(marker: u8) -> String {
    match marker {
        0xD8 => "SOI, start of image, 影像起始",
        0xD9 => "EOI, end of image, 影像結尾",
        0xE0 => "APP0, 記錄影像的長、寬等等基本資訊",
        0xDB => "DQT, define quantization table, 定義量化表",
        0xC4 => "DHT, define huffman table, 定義霍夫曼表",
        0xDA => "SOS, start of scan, 壓縮的數據由此開始",
        0xC0 => "SOF, start of frame(baseline)",
        0xFE => "COM, comment, 註解",
        _ => "不知道是什麼，可能標準書有，但是本程式不支援"
    }.to_string()
}

pub fn marker_detector(mut reader: BufReader<File>) -> std::io::Result<()> {

    let mut c: [u8; 1] = [0; 1];

    loop {
        reader.read(&mut c)?;
        if c[0] != 0xFF {
            continue;
        }

        reader.read(&mut c)?;
        match c[0] {
            0x00 => {
                // 0xFF 後綴 0x00 並不代表 marker
            }
            m => {
                println!("marker: {:#X?}, recognized as {}", m, marker_info(m));
            }
        }
        if c[0] == 0xD9 {
            break;
        }
    }
    Ok(())
}