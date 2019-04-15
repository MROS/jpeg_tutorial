#![allow(non_snake_case)]

use std::io::BufReader;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::default::Default;
use std::io::SeekFrom;

use crate::primitives::*;

use std::collections::HashMap;

const MARKER_PREFIX: u8 = 0xFF;

const SOI_MARKER: u8 = 0xD8;      // start of image, 圖片起始
const EOI_MARKER: u8 = 0xD9;      // end of image, 圖片結束

const APP0_MARKER: u8 = 0xE0;     // APP0, JFIF 的額外資訊

const DQT_MARKER: u8 = 0xDB;      // DQT, define quantization table, 定義量化表
const DHT_MARKER: u8 = 0xC4;      // DHT, define huffman table, 定義霍夫曼表

const SOF0_MARKER: u8 = 0xC0;     // SOF, start of frame(baseline)
const SOS_MARKER: u8 = 0xDA;      // SOS, start of scan, 壓縮的數據由此開始

// const COM_MARKER: u8 = 0xFE;      // COM, comment, 註解

const DC: u8 = 0x00;
const AC: u8 = 0x01;


fn read_u8(reader:&mut BufReader<File>) -> u8 {
    let mut c: [u8; 1] = [0; 1];
    reader.read_exact(&mut c).expect("read_u8 失敗");
    return c[0];
}

fn read_u16(reader: &mut BufReader<File>) -> u16 {
    let mut c: [u8; 2] = [0; 2];
    reader.read_exact(&mut c).expect("read_u16 失敗");
    return (c[0] as u16) * 256 + c[1] as u16;
}


fn read_app0(reader: &mut BufReader<File>) -> AppInfo {
    let len = read_u16(reader);
    println!("區段長度 {} bytes", len);
    let mut app_info: AppInfo = Default::default();
    reader.read_exact(&mut app_info.identifier).expect("read_app0 失敗");

    app_info.version_major_id = read_u8(reader);
    app_info.version_minor_id = read_u8(reader);

    app_info.units = read_u8(reader);

    app_info.x_density = read_u16(reader);
    app_info.y_density = read_u16(reader);

    app_info.x_thumbnail = read_u8(reader);
    app_info.y_thumbnail = read_u8(reader);

    // 不管 thumbnail
    let thumbnail_length: i64 = 3 * (app_info.x_thumbnail as i64) * (app_info.y_thumbnail as i64);
    reader.seek(SeekFrom::Current(thumbnail_length)).expect("seek in read_app0 失敗");

    return app_info;
}

// 回傳值爲 (AC/DC, 編號, 霍夫曼表)
// 使用 hashmap 來儲存碼字與信源符號的對應關係
// 001 跟 1 雖然在編碼上代表不同含義，但是無法在數字上表示出來
// 必須再增加一個編碼長度的資訊，才能夠分辨開來，所以我將一個編碼表示爲一個 tuple
// 譬如說 001 使用 (3, 1) 來表示長度爲 3 ，數值爲 1 ，而 1 則用 (1, 1) 來表示長度爲 1 ，數值爲 1
// 有很多優化的方式，例如使用二元樹，但本處爲了程式碼清晰不優化
fn read_dht(reader: &mut BufReader<File>) -> Vec<(u8, u8, HashMap<(u8, u16), u8>)> {
    let mut ret = Vec::new();

    let mut len = read_u16(reader);
    println!("區段長度 {} bytes", len);
    len -= 2;
    while len > 0 {
        let c = read_u8(reader);
        let ac_dc = c >> 4;
        let id = c & 0x0F;
        let mut map = HashMap::new();
        let mut height_info: [u8; 16] = [0; 16];
        reader.read_exact(&mut height_info).expect("read height_info in dqt 失敗");
        println!("高度（碼字長度）分配 {:?}", height_info);
        len -= 17;

        let mut code = 0;
        for h in 0..16 {
            for _ in 0..height_info[h] {
                let source_symbol = read_u8(reader);
                map.insert(((h + 1) as u8, code), source_symbol);

                // 下幾行只是要讓輸出好看點
                let binary = format!("{:b}", code);
                let zeros = (h + 1) - binary.len();
                println!("{}{} {:#04X}", "0".repeat(zeros), binary, source_symbol);

                code += 1;
                len -= 1;
            }
            code <<= 1;            // *= 2
        }
        ret.push((ac_dc, id, map));
    }

    return ret;
}

fn read_dqt(reader: &mut BufReader<File>) -> Vec<(usize, [f32; 64])> {
    let mut len = read_u16(reader);
    println!("區段長度 {} bytes", len);
    len -= 2;  // 扣掉自身長度

    let mut tables = Vec::new();
    // 一個 dqt 區段中，可能包含多個量化表
    while len > 0 {
        let c = read_u8(reader);
        let id = c & 0x0F;
        let precision = c >> 4;
        println!("量化表 {} ，精度爲 {}", id, precision);

        let mut table = [0.0; 64];
        if precision == 0 {
            for i in 0..64 {
                table[i] = f32::from(read_u8(reader));
            }
            len -= 65
        } else if precision == 1 {
            for i in 0..64 {
                table[i] = f32::from(read_u16(reader));
            }
            len -= 129;
        } else {
            println!("量化表 {} 精度爲 {}，不符合規範", id, precision);
        }

        for i in 0..8 {
            for j in 0..8 {
                print!("{:2} ", table[i*8 + j]);
            }
            println!("");
        }
        tables.push((id as usize, table));
    }

    return tables;
}

fn read_sof0_component(reader: &mut BufReader<File>) -> ComponentInfo {
    let mut sof_component: ComponentInfo = Default::default();
    let c = read_u8(reader);
    sof_component.horizontal_sampling = c >> 4;
    sof_component.vertical_sampling = c & 0x0F;
    sof_component.quant_table_id = read_u8(reader);

    return sof_component;
}

fn read_sof0(reader: &mut BufReader<File>) -> SofInfo {
    let len = read_u16(reader);
    println!("區段長度 {} bytes", len);
    let mut sof_info: SofInfo = Default::default();
    sof_info.precision = read_u8(reader);
    sof_info.height = read_u16(reader);
    sof_info.width = read_u16(reader);

    let number_of_component = read_u8(reader);

    for _ in 0..number_of_component {
        let component_id = read_u8(reader);
        // Y => 1, Cb => 2, Cr => 3 ，將它們減 1 使的陣列索引從 0 開始
        sof_info.component_infos[component_id as usize - 1] = read_sof0_component(reader);
    }
    sof_info.max_horizontal_sampling = sof_info.component_infos.iter().map(|x| { x.horizontal_sampling }).max().unwrap();
    sof_info.max_vertical_sampling = sof_info.component_infos.iter().map(|x| { x.vertical_sampling }).max().unwrap();
    println!("max horizontal sampling: {}, max vertical sampling: {}", sof_info.max_horizontal_sampling, sof_info.max_vertical_sampling);
    return sof_info;
}

struct BitStream<'a> {
    reader: &'a mut BufReader<File>,
    buf: u8,
    count: u8,
    last_dc: [f32; 3],
}

impl<'a> BitStream<'a> {
    fn new(reader: &'a mut BufReader<File>) -> BitStream {
        BitStream {
            reader: reader,
            buf: 0,
            count: 0,
            last_dc: [0.0, 0.0, 0.0]
        }
    }
    fn get_a_bit(&mut self) -> u8 {
        if self.count == 0 {
            self.buf = read_u8(self.reader);
            if self.buf == 0xFF {
                let check = read_u8(self.reader);
                assert!(check == 0x00, "壓縮圖像數據中 0xFF 不接 0x00");
            }
        }
        let ret = if (self.buf & (1 << (7 - self.count))) > 0 { 1 } else { 0 };
        self.count = if self.count == 7 { 0 } else { self.count + 1 };
        return ret;
    }
    fn matchHuffman(&mut self, map: &HashMap<(u8, u16), u8>) -> u8 {
        let mut code: u16 = 0;
        let mut len: u8 = 1;
        loop {
            code = code << 1;
            code += self.get_a_bit() as u16;
            if map.contains_key(&(len, code)) {
                return *map.get(&(len, code)).unwrap();
            }
            len += 1;
            if len > 16 {
                panic!("無法霍夫曼解碼");
            }
        }
    }
    fn read_value(&mut self, code_len: u8) -> f32 {
        let mut ret: i16 = 1;
        let first = self.get_a_bit();
        for _ in 1..code_len {
            let b = self.get_a_bit();
            ret = ret << 1;
            ret += if first == b { 1 } else { 0 };
        }
        ret = if first == 1 { ret } else{ -ret };
        return ret as f32;
    }
    fn read_dc(&mut self, map: &HashMap<(u8, u16), u8>, id: usize) -> f32 {
        let code_len = self.matchHuffman(map);
        if code_len == 0 { return self.last_dc[id]; }
        self.last_dc[id] += self.read_value(code_len);
        return self.last_dc[id];
    }
    fn read_ac(&mut self, map: &HashMap<(u8, u16), u8>) -> AcValue {
        let code_len = self.matchHuffman(map);
        match code_len {
            0x00 => AcValue::AllZeros,
            0xF0 => AcValue::SixteenZeros,
            x => {
                AcValue::Normal {
                    zeros: (x >> 4) as usize,
                    value: self.read_value(x & 0x0F)
                }
            }
        }
    }
}

enum AcValue {
    SixteenZeros,
    AllZeros,
    Normal { zeros: usize, value: f32 } 
}

// TODO: 在 read_mcus 時就拿出各 component 的 table ，避免重複計算
fn read_mcu(bits: &mut BitStream, jpeg_meta_data: &JPEGMetaData) -> MCU {
    let component_infos = &jpeg_meta_data.sof_info.component_infos;
    let table_mapping = jpeg_meta_data.table_mapping;

    let mut mcu: MCU = Default::default();
    // id 遍歷 Y, Cr, Cb
    for id in 0..3 {
        let height = component_infos[id].vertical_sampling as usize;
        let width = component_infos[id].horizontal_sampling as usize;
        let mut blocks: Vec<Vec<Block>> = vec![vec![Default::default(); width]; height];
        let dc_table = &jpeg_meta_data.huffman_tables.dc_tables[table_mapping[id].0];
        let ac_table = &jpeg_meta_data.huffman_tables.ac_tables[table_mapping[id].1];
        for h in 0..height {
            for w in 0..width {
                // 讀取一個 block
                blocks[h][w][0][0] = bits.read_dc(dc_table, id);
                let mut count = 1;
                while count < 64 {
                    let ac_value = bits.read_ac(ac_table);
                    match ac_value {
                        AcValue::SixteenZeros => {
                            for _ in 0..16 {
                                blocks[h][w][count / 8][count % 8] = 0.0;
                                count += 1;
                            }
                        }
                        AcValue::AllZeros => {
                            while count < 64 {
                                blocks[h][w][count / 8][count % 8] = 0.0;
                                count += 1;
                            }
                        }
                        AcValue::Normal { zeros, value } => {
                            for _ in 0..zeros {
                                blocks[h][w][count / 8][count % 8] = 0.0;
                                count += 1;
                            }
                            blocks[h][w][count / 8][count % 8] = value;
                            count += 1;
                        }
                    }
                }
            }
        }
        mcu[id] = blocks;
    }
    return mcu;
}

fn read_mcus(reader: &mut BufReader<File>, jpeg_meta_data: &JPEGMetaData) -> Vec<Vec<MCU>> {
    let sof_info = &jpeg_meta_data.sof_info;
    let image_width = sof_info.width;
    let image_height = sof_info.height;

    let w = (image_width - 1) / (8 * sof_info.max_horizontal_sampling as u16) + 1;     // 寬度上有 w 個 MCU
    let h = (image_height - 1) / (8 * sof_info.max_vertical_sampling as u16) + 1;   // 高度上有 h 個 MCU
    println!("寬度上有 {} 個 MCU ，高度上有 {} 個 MCU", w, h);

    let mut bits = BitStream::new(reader);
    let mut MCUs = vec![vec![Default::default(); w as usize]; h as usize];
    for i in 0..h {
        for j in 0..w {
            MCUs[i as usize][j as usize] = read_mcu(&mut bits, jpeg_meta_data);
        }
    }
    return MCUs;
}

fn component_name(id: u8) -> &'static str {
    match id {
        1 => "Y",
        2 => "Cb",
        3 => "Cr",
        _ => panic!("不知名的顏色分量 id: {}", id)
    }
}

// 在讀取 read_sos 之前， jpeg_data 的其他欄位都讀取好了
fn read_sos(reader: &mut BufReader<File>) -> [(usize, usize); 3] {
    let len = read_u16(reader);
    println!("區段長度 {} bytes", len);

    let mut table_mapping: [(usize, usize); 3] = Default::default();

    let component_number = read_u8(reader);
    assert!(component_number == 3);

    for _ in 0..3 {
        let component = read_u8(reader);
        let id = read_u8(reader);
        let dc_id = (id >> 4) as usize;
        let ac_id = (id & 0x0F) as usize;
        println!("{} 顏色色分量，直流霍夫曼表 id = {}, 交流霍夫曼表 id = {}", component_name(component), dc_id, ac_id);
        // Y => 1, Cb => 2, Cr => 3 ，將它們減 1 使的陣列索引從 0 開始
        table_mapping[component as usize - 1] = (dc_id, ac_id);
    }

    // 接下來 3 bytes 在 baseline 是固定的，直接 seek 過也可
    let mut c = read_u8(reader);
    assert!(c == 0x00);
    c = read_u8(reader);
    assert!(c == 0x3F);
    c = read_u8(reader);
    assert!(c == 0x00);

    return table_mapping;
}

// fn read_useless_section(reader: &mut BufReader<File>) {
//     let len = read_u16(reader);
//     println!("len {}", len);
//     reader.seek(SeekFrom::Current((len - 2) as i64));
// }

pub fn data_reader(mut reader: BufReader<File>) -> (JPEGMetaData, Vec<Vec<MCU>>) {

    let mut jpeg_meta_data: JPEGMetaData = JPEGMetaData::new();
    let mut MCUs = Default::default();

    loop {
        let mut c = read_u8(&mut reader);
        if c != MARKER_PREFIX {
            continue;
        }

        c = read_u8(&mut reader);
        match c {
            SOI_MARKER => {
                println!("==================  掃過 SOI ，圖片起始 ====================");
            },
            EOI_MARKER => {
                println!("==================  掃過 EOI ，圖片結束 ====================");
                break;
            },
            SOF0_MARKER => {
                println!("==================  掃過 SOF ====================");
                jpeg_meta_data.sof_info = read_sof0(&mut reader);
                println!("sof_info: {:#?}", jpeg_meta_data.sof_info);
            },
            SOS_MARKER => {
                println!("==================  掃過 SOS ====================");
                jpeg_meta_data.table_mapping = read_sos(&mut reader);
                MCUs = read_mcus(&mut reader, &jpeg_meta_data);
            },
            APP0_MARKER => {
                println!("==================  掃過 APP0 ====================");
                jpeg_meta_data.app_info = read_app0(&mut reader);
                println!("app_info: {:#?}", jpeg_meta_data.app_info);
            },
            DHT_MARKER => {
                println!("==================  掃過 DHT ====================");
                let huffman_tables = read_dht(&mut reader);
                for (ac_dc, id, table) in huffman_tables {
                    match ac_dc {
                        AC => {
                            jpeg_meta_data.huffman_tables.ac_tables[id as usize] = table;
                        },
                        DC => {
                            jpeg_meta_data.huffman_tables.dc_tables[id as usize] = table;
                        },
                        _ => {
                            panic!("DHT 非直流也非交流");
                        }
                    }
                }
            },
            DQT_MARKER => {
                println!("==================  掃過 DQT ====================");
                for (id, table) in read_dqt(&mut reader) {
                    jpeg_meta_data.quant_tables[id] = table;
                }
            },
            m => {
                println!("other marker: {:#X?}", m);
                // read_useless_section(&mut reader);
            }
        }
    }
    return (jpeg_meta_data, MCUs);
}