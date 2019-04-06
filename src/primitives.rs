use std::collections::HashMap;

pub struct JPEGMetaData {
    pub app_info: AppInfo,
    pub sof_info: SofInfo,
    pub huffman_tables: HuffmanTable,
    pub quant_tables: [[u16; 64]; 4],
    pub table_mapping: [(usize, usize); 3]
}

impl JPEGMetaData {
    pub fn new() -> JPEGMetaData {
        JPEGMetaData {
            app_info: Default::default(),
            sof_info: Default::default(),
            huffman_tables: Default::default(),
            quant_tables: [[0; 64]; 4],
            table_mapping: Default::default()
        }
    }
}

#[derive(Clone, Default)]
pub struct Block(pub [[f32; 8]; 8]);

impl Block {
    fn quantify(&self) {

    }
}

pub type MCU = [Vec<Vec<Block>>; 3];

#[derive(Default, Debug)]
pub struct HuffmanTable {
    pub dc_tables: [HashMap<(u8, u16), u8>; 2],
    pub ac_tables: [HashMap<(u8, u16), u8>; 2]
}

#[derive(Default, Debug)]
pub struct ComponentInfo {
    pub horizontal_sampling: u8,
    pub vertical_sampling: u8,
    pub quant_table_id: u8
}

#[derive(Default, Debug)]
pub struct SofInfo {
    pub precision: u8,
    pub height: u16,
    pub width: u16,
    pub component_infos: [ComponentInfo; 3]
}

// APP0 section 下的資料
#[derive(Default, Debug)]
pub struct AppInfo {
    pub identifier: [u8; 5],
    pub version_major_id: u8,
    pub version_minor_id: u8,
    pub units: u8,
    pub x_density: u16,
    pub y_density: u16,
    pub x_thumbnail: u8,
    pub y_thumbnail: u8,
    // thumbnail: Vec<u8>      // 別管 thumbnail 了，解碼用不上
}