mod image;
mod display;
mod decoder;
mod marker;
mod reader;
mod primitives;

use image::Image;
use display::display_image;
use decoder::decoder;
use marker::marker_detector;
use reader::data_reader;

use std::fs::File;
use std::io::BufReader;

extern crate clap;
use clap::{Arg, App};

fn main() -> std::io::Result<()> {
    let matches = App::new("JPEG tutorial")
                          .author("MROS. <yc1043@gmail.com>")
                          .about("跟我寫 JPEG 解碼器")
                          .arg(Arg::with_name("path")
                               .help("要解析的 JPEG 檔案路徑")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("marker")
                               .short("m")
                               .long("marker")
                               .multiple(true)
                               .help("僅打印 marker"))
                          .arg(Arg::with_name("reader")
                               .short("r")
                               .long("reader")
                               .multiple(true)
                               .help("僅解碼檔案，不顯示"))
                          .get_matches();

    let filename = matches.value_of("path").unwrap();

    let f = File::open(filename)?;
    let reader = BufReader::new(f);

    if matches.is_present("marker") {
        marker_detector(reader)?;
    } else if matches.is_present("reader") {
        data_reader(reader);
    } else {
        // 沒有額外參數，直接解碼並顯示
        let image: Image = decoder(reader);
        display_image(image);
    }

    Ok(())
}