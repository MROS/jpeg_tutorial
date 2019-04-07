mod image;
mod ppm;
mod decoder;
mod marker;
mod reader;
mod primitives;

use image::Image;
use ppm::to_ppm;
use decoder::{decoder,show_mcu_stage};
use marker::marker_detector;
use reader::data_reader;

use std::fs::File;
use std::io::BufReader;

use std::str::FromStr;

extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() -> std::io::Result<()> {
    let matches = App::new("JPEG tutorial")
                          .author("MROS. <yc1043@gmail.com>")
                          .about("「跟我寫 JPEG 解碼器」系列文配套程式碼")
                          .arg(Arg::with_name("path")
                               .help("JPEG 檔的路徑")
                               .index(1)
                               .required(true))
                          .subcommand(SubCommand::with_name("marker")
                               .about("僅打印 marker"))
                          .subcommand(SubCommand::with_name("reader")
                               .about("解碼檔案並打印各區段"))
                          .subcommand(SubCommand::with_name("ppm")
                               .about("輸出 ppm 格式，檔名固定爲 out.ppm"))
                          .subcommand(SubCommand::with_name("mcu")
                               .about("打印 mcu 解碼的各階段")
                               .arg(Arg::with_name("y")
                                    .help("想取得的 mcu 的縱座標，由上往下")
                                    .required(true)
                                    .index(1))
                               .arg(Arg::with_name("x")
                                    .help("想取得的 mcu 的橫座標，由左往右")
                                    .required(true)
                                    .index(2)))
                          .get_matches();

    let filename = matches.value_of("path").unwrap();

    let f = File::open(filename)?;
    let reader = BufReader::new(f);

    match matches.subcommand_name() {
        Some("marker") => {
            marker_detector(reader)?;
        }
        Some("reader") => {
            data_reader(reader);
        }
        Some("ppm") => {
            let image: Image = decoder(reader);
            to_ppm(image)?;
        }
        Some("mcu") => {
            let y = matches.subcommand_matches("mcu").unwrap().value_of("y").unwrap();
            let x = matches.subcommand_matches("mcu").unwrap().value_of("x").unwrap();
            println!("{} {}", y, x);
            show_mcu_stage(reader, FromStr::from_str(y).unwrap(), FromStr::from_str(x).unwrap());
        }
        None        => {
            let image: Image = decoder(reader);
            to_ppm(image)?;
        }
        Some(_) => {
            println!("unrechable");
        }
    }

    Ok(())
}