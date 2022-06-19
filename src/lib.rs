use image::{io::Reader, ImageFormat};
use rayon::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn converter(input: &Path, output: &Path, mode: &str, format: ImageFormat) {
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

pub fn folder_convert(input: &Path, output: &Path, mode: &str, format: ImageFormat) {
    let mod_output: PathBuf;
    match fs::create_dir_all(output.join("rustic-converted")) {
        Ok(_) => mod_output = output.join("rustic-converted"),
        Err(err) => {
            eprintln!("Error creating output folder: {}", err);
            return;
        }
    }

    if output.is_file() {
        eprintln!("Error: The output is a file")
    } else if output.is_dir() {
        match fs::read_dir(input) {
            Ok(entries) => {
                let entries: Vec<_> = entries.collect();
                entries.par_iter().for_each(|entry| match entry {
                    Ok(entry) => {
                        let mut par_item = Vec::new();
                        if entry.path().is_file() {
                            par_item.push(entry.path());
                        } else {
                            println!("Ignoring folder: {}", entry.path().display());
                        }
                        if par_item.len() > 0 {
                            par_item.par_iter().for_each(|item| {
                                converter(item, &mod_output, mode, format);
                            });
                        } else {
                            eprintln!("Error: No files found in folder");
                        }
                    }
                    Err(err) => {
                        eprintln!("Error reading entry: {}", err);
                        return;
                    }
                });
            }
            Err(err) => {
                eprintln!("Error reading input folder: {}", err);
                return;
            }
        }
    }
}

pub fn convert(input: &Path, output: &Path, mode: &str, format: ImageFormat) {
    if input == output {
        eprintln!("Error: The input and output files are the same");
        return;
    }
    if input.is_dir() {
        eprintln!("Error: The input is a folder");
        return;
    } else {
        converter(input, output, mode, format);
    }
}

pub fn image_formatter(mode: &str) -> Result<ImageFormat, &str> {
    if mode == "png" {
        Ok(ImageFormat::Png)
    } else if mode == "jpg" || mode == "jpeg" {
        Ok(ImageFormat::Jpeg)
    } else if mode == "gif" {
        Ok(ImageFormat::Gif)
    } else if mode == "bmp" {
        Ok(ImageFormat::Bmp)
    } else {
        Err("Unsupported image format")
    }
}
