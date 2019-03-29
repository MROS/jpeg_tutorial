mod image;
mod display;
mod decoder;
mod marker;

use image::Image;
use display::display_image;
use decoder::decoder;
use marker::marker_detector;

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
                               .help("是否僅打印 marker"))
                          .get_matches();
    // let args: Vec<String> = env::args().collect();
    // let length = args.len();
    // if length != 2 {
    //     println!("用法：cargo run XXX.jpg");
    //     return Ok(());
    // }
    let filename = matches.value_of("path").unwrap();

    let f = File::open(filename)?;
    let reader = BufReader::new(f);

    if matches.is_present("marker") {

        marker_detector(reader);

    } else {
        // 沒有額外參數，直接解碼並顯示
        let image: Image = decoder(reader);
        display_image(image);
    }

    Ok(())
}