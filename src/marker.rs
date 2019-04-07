use std::io::BufReader;
use std::fs::File;
use std::io::Read;

fn marker_info(marker: u8) -> String {
    match marker {
        0xE0 => "APP0, JFIF 的額外資訊",
        0xDB => "DQT, define quantization table, 定義量化表",
        0xC4 => "DHT, define huffman table, 定義霍夫曼表",
        0xC0 => "SOF0, start of frame(baseline)",
        0xDA => "SOS, start of scan, 壓縮的數據由此開始",
        _ => "本程式不支援"
    }.to_string()
}

fn read_u16(reader: &mut BufReader<File>) -> u16 {
    let mut c: [u8; 2] = [0; 2];
    reader.read_exact(&mut c).expect("read_u16 失敗");
    return (c[0] as u16) * 256 + c[1] as u16;
}

// 分析檔頭、
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
            },
            0xD8 => {
                println!("SOI, start of image, 圖像起始");
            }
            0xD9 => {
                println!("EOI, end of image, 圖像結尾");
            }
            m => {
                let length = read_u16(&mut reader);
                println!("marker: {:#X?}, recognized as {}, 長度爲 {}", m, marker_info(m), length);
            }
        }
        if c[0] == 0xD9 {
            break;
        }
    }
    Ok(())
}