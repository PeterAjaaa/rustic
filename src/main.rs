use clap::Parser;
use image::{io::Reader, ImageFormat};
use std::path::Path;

fn main() {
    #[derive(Parser)]
    #[clap(version, about)]
    pub struct Arguments {
        #[clap(short = 'i', long = "input", display_order = 1)]
        /// Path to input file or folder containing images
        input: String,
        #[clap(short = 'o', long = "output", display_order = 2)]
        /// Path to output file or folder containing images (file extension is not mandatory)
        output: String,
        #[clap(short = 'm', long = "mode", display_order = 3)]
        /// Mode of image conversion (supported: "png", "jpg (or jpeg)", "bmp", "gif")
        mode: String,
    }

    let args = Arguments::parse();
    let input = Path::new(&args.input);
    let output = Path::new(&args.output);
    let mode = &args.mode;
    let format: ImageFormat;

    if mode == "png" {
        format = ImageFormat::Png;
    } else if mode == "jpg" || mode == "jpeg" {
        format = ImageFormat::Jpeg;
    } else if mode == "gif" {
        format = ImageFormat::Gif;
    } else if mode == "bmp" {
        format = ImageFormat::Bmp;
    } else {
        println!("Error: Unsupported image format");
        return;
    }

    if input.is_file() {
        if input == output {
            println!("Error: The input and output files are the same");
            return;
        } else if input.is_dir() {
            println!("Error: The input is a folder");
            return;
        } else {
            convert(input, output, mode)
        }
    }
}

fn convert(input: &Path, output: &Path, mode: &str) {
    match Reader::open(input) {
        Ok(reader) => match reader.with_guessed_format() {
            Ok(reader) => match reader.decode() {
                Ok(image) => {
                    let modif = format!(
                        "{}-converted.{}",
                        input.file_name().unwrap().to_string_lossy(),
                        mode
                    );
                    let mod_output = output.join(modif);
                    match image.save(&mod_output) {
                        Ok(_) => {
                            println!("{} saved to {}", input.display(), mod_output.display());
                        }
                        Err(err) => {
                            println!("Error saving image: {}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("Error decoding image: {}", err);
                }
            },
            Err(err) => {
                println!("Error analyzing image format: {}", err);
            }
        },
        Err(err) => {
            println!("Error reading file: {}", err);
        }
    }
}
