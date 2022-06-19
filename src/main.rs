use clap::Parser;
use image::ImageFormat;
use rustic::{convert, folder_convert, image_formatter};
use std::{path::Path, time::Instant};

fn main() {
    let before = Instant::now();
    #[derive(Parser)]
    #[clap(version, about)]
    struct Arguments {
        #[clap(short = 'i', long = "input", display_order = 1)]
        /// Path to input file or folder containing images
        input: String,
        #[clap(short = 'o', long = "output", display_order = 2)]
        /// Path to output file or folder containing images (file extension is not mandatory)
        output: String,
        #[clap(short = 'm', long = "mode", display_order = 3)]
        /// Mode of image convert (supported: "png", "jpg (or jpeg)", "bmp", "gif")
        mode: String,
    }

    let args = Arguments::parse();
    let input = Path::new(&args.input);
    let output = Path::new(&args.output);
    let mode = &args.mode;

    let format: ImageFormat = match image_formatter(mode) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    if input.is_file() {
        convert(input, output, mode, format)
    } else if input.is_dir() {
        folder_convert(input, output, mode, format)
    } else {
        eprintln!("Error: The output is neither a file nor a folder");
        return;
    }
    println!("Done in {}ms", before.elapsed().as_millis());
}
