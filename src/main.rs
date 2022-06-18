use clap::Parser;
use image::{io::Reader, ImageFormat};
use std::{fs, path::Path};

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
            eprintln!("Error: The input and output files are the same");
            return;
        }
        if input.is_dir() {
            eprintln!("Error: The input is a folder");
            return;
        } else {
            convert(input, output, mode, format);
        }
    } else if input.is_dir() {
        match fs::create_dir_all(&output) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error: Could not create output folder");
                return;
            }
        }
        if output.is_file() {
            eprintln!("Error: The output is a file")
        }
    }
}

fn convert(input: &Path, output: &Path, mode: &str, format: ImageFormat) {
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
                    match image.save_with_format(&mod_output, format) {
                        Ok(_) => {
                            println!("{} saved to {}", input.display(), mod_output.display());
                        }
                        Err(err) => {
                            eprintln!("Error saving image: {}", err);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error decoding image: {}", err);
                }
            },
            Err(err) => {
                eprintln!("Error analyzing image format: {}", err);
            }
        },
        Err(err) => {
            eprintln!("Error reading file: {}", err);
        }
    }
}
