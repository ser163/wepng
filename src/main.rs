extern crate image;

use std::env;
use std::path::Path;
use image::ImageFormat;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} INPUT", args[0]);
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    let output_path = input_path.with_extension("png");

    // 读取图片
    let img = match image::open(input_path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // 写入图片
    if let Err(e) = img.save_with_format(output_path, ImageFormat::Png) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}