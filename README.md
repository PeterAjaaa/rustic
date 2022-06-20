# Rustic (Rust Image Converter)

Rustic is a naive (but blazingly fast) image converter written in Rust. Rustic supports conversion between PNG, JPEG, GIF, and BMP image formats. Rustic comes in a simple and easy to use command line interface.

## Features

- Single and batch conversion of images.

- Easy to use command line interface.

- Portable and cross-platform.

- Faster execution speed compared to web-based converter.

- PNG, JPEG, GIF, and BMP conversion support.

## Installation

`Rustic` is a portable and cross-platform application. Meaning that you don't need to do installation on your computer. You just need to download the `rustic` executable file and run it. Currently, `rustic` is available for Linux, and Windows.

- Go to [Rustic Repository on GitHub](https://github.com/NightlyWare/rustic)

- Locate the "Packages" on the left side of the page.

- CLick on that and download the appropriate package for your operating system.

## Building

To build `rustic` you need to have Rust installed on your system.

- Go to [Rustic Repository on GitHub](https://github.com/NightlyWare/rustic)

- Do a `git clone` of the repository, using:

    `git clone https://github.com/NightlyWare/rustic`

- Run the following command in the Rustic directory:

    `cargo build --release`

- You should see a `rustic` executable file in the `/target/release` directory.

    `target/release/rustic`

- Note that based on your operating system, the executable file extension may be different.

## Usage

When first running `rustic`, it is advised to run it with the `--help` or the `-h` flag.

    `rustic --help`

Or using the shorthand flags:

    `rustic -h`

## ToDo

-[] Add support for WebP image format.

## Changelog

See the [CHANGELOG](CHANGELOG.md) for the list of changes.
