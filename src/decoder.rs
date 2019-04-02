use crate::image::Image;

use std::io::BufReader;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::default::Default;
use std::io::SeekFrom;

const MARKER_PREFIX: u8 = 0xFF;

const SOI_MARKER: u8 = 0xD8;      // start of image, 圖片起始
const EOI_MARKER: u8 = 0xD9;      // end of image, 圖片結束

const APP0_MARKER: u8 = 0xE0;     // APP0, JFIF 的額外資訊

const DQT_MARKER: u8 = 0xDB;      // DQT, define quantization table, 定義量化表
const DHT_MARKER: u8 = 0xC4;      // DHT, define huffman table, 定義霍夫曼表

const SOF0_MARKER: u8 = 0xC0;      // SOF, start of frame(baseline)
const SOS_MARKER: u8 = 0xDA;      // SOS, start of scan, 壓縮的數據由此開始

// const COM_MARKER: u8 = 0xFE;      // COM, comment, 註解

#[derive(Default)]
struct MetaData {
    app_info: AppInfo,
    sof_info: SofInfo,
    // quant_table: [[u16; 64]; 4]
}

#[derive(Default, Debug)]
struct SofComponent {
    horizontal_sampling: u8,
    vertical_sampling: u8,
    quant_table_id: u8
}

#[derive(Default, Debug)]
struct SofInfo {
    precision: u8,
    height: u16,
    width: u16,
    Y: SofComponent,
    Cb: SofComponent,
    Cr: SofComponent
}


// APP0 section 下的資料
#[derive(Default, Debug)]
struct AppInfo {
    identifier: [u8; 5],
    version_major_id: u8,
    version_minor_id: u8,
    units: u8,
    x_density: u16,
    y_density: u16,
    x_thumbnail: u8,
    y_thumbnail: u8,
    // thumbnail: Vec<u8>      // 別管 thumbnail 了，解碼用不上
}

fn read_u8(reader:&mut BufReader<File>) -> u8 {
    let mut c: [u8; 1] = [0; 1];
    reader.read_exact(&mut c);
    return c[0];
}

fn read_u16(reader: &mut BufReader<File>) -> u16 {
    let mut c: [u8; 2] = [0; 2];
    reader.read_exact(&mut c);
    return (c[0] as u16) * 256 + c[1] as u16;
}


fn read_app0(reader: &mut BufReader<File>) -> AppInfo {
    let len = read_u16(reader);
    println!("len {}", len);
    let mut app_info: AppInfo = Default::default();
    reader.read_exact(&mut app_info.identifier);

    app_info.version_major_id = read_u8(reader);
    app_info.version_minor_id = read_u8(reader);

    app_info.units = read_u8(reader);

    app_info.x_density = read_u16(reader);
    app_info.y_density = read_u16(reader);

    app_info.x_thumbnail = read_u8(reader);
    app_info.y_thumbnail = read_u8(reader);

    // 不管 thumbnail
    let thumbnail_length: i64 = 3 * (app_info.x_thumbnail as i64) * (app_info.y_thumbnail as i64);
    reader.seek(SeekFrom::Current(thumbnail_length));

    return app_info;
}

fn read_dqt(reader: &mut BufReader<File>) -> (usize, [u16; 64]) {
    let len = read_u16(reader);
    println!("len {}", len);
    let c = read_u8(reader);
    let id = c & 0x0F;
    let precision = c >> 4;
    println!("量化表 {} ，精度爲 {}", id, precision);

    let mut table = [0; 64];
    if precision == 0 {
        for i in 0..64 {
           table[i] = read_u8(reader) as u16;
        }
    } else if precision == 1 {
        for i in 0..64 {
           table[i] = read_u16(reader);
        }
    } else {
        println!("量化表 {} 精度爲 {}，不符合規範", id, precision);
    }

    for i in 0..8 {
        for j in 0..8 {
            print!("{:2} ", table[i*8 + j]);
        }
        println!("");
    }

    return (id as usize, table);
}

fn read_sof0_component(reader: &mut BufReader<File>) -> SofComponent {
    let mut sof_component: SofComponent = Default::default();
    let c = read_u8(reader);
    sof_component.horizontal_sampling = c >> 4;
    sof_component.vertical_sampling = c & 0x0F;
    sof_component.quant_table_id = read_u8(reader);

    return sof_component;
}

fn read_sof0(reader: &mut BufReader<File>) -> SofInfo {
    let len = read_u16(reader);
    println!("len {}", len);
    let mut sof_info: SofInfo = Default::default();
    sof_info.precision = read_u8(reader);
    sof_info.height = read_u16(reader);
    sof_info.width = read_u16(reader);

    let number_of_component = read_u8(reader);

    for i in 0..number_of_component {
        let component_id = read_u8(reader);
        match component_id {
            1 => {
                sof_info.Y = read_sof0_component(reader);
            },
            2 => {
                sof_info.Cb = read_sof0_component(reader);
            },
            3 => {
                sof_info.Cr = read_sof0_component(reader);
            },
            other => {
                println!("不符合規範的 SOF0 component: {}", other);
            }
        }
    }
    return sof_info;
}
// fn read_useless_section(reader: &mut BufReader<File>) {
//     let len = read_u16(reader);
//     println!("len {}", len);
//     reader.seek(SeekFrom::Current((len - 2) as i64));
// }

pub fn decoder(mut reader: BufReader<File>) -> Image {
    let mut c: [u8; 1] = [0; 1];

    let mut meta_data: MetaData = Default::default();

    loop {
        reader.read(&mut c);
        if c[0] != MARKER_PREFIX {
            // println!("????");
            continue;
        }

        reader.read(&mut c);
        match c[0] {
            SOI_MARKER => {
                println!("掃過 SOI marker ，圖片起始");
            },
            EOI_MARKER => {
                println!("掃過 EOI marker ，圖片結束");
                break;
            },
            SOF0_MARKER => {
                println!("掃過 SOF marker");
                meta_data.sof_info = read_sof0(&mut reader);
                println!("sof_info: {:#?}", meta_data.sof_info);
                break;
            }
            APP0_MARKER => {
                println!("掃過 APP0 marker");
                meta_data.app_info = read_app0(&mut reader);
                println!("app_info: {:#?}", meta_data.app_info);
            },
            DQT_MARKER => {
                println!("掃過 DQT marker");
                read_dqt(&mut reader);
            }
            0x00 => {

            }
            m => {
                println!("other marker: {:#X?}", m);
                // read_useless_section(&mut reader);
            }
        }

    }

    return Image::new(800, 600);
}