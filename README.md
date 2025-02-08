# Image to .webp CLI

This is a simple CLI to convert images to .webp format. This project is not intended to be a full-featured image converter, but a simple tool to convert images to .webp format. This is a small personal test project to learn Rust.

## Installation

```bash
git clone https://github.com/edugzlez/image-to-webp-converter.git
cd image-to-webp-converter
cargo build --release
```

If you want to get the binary into your system:
```bash
cp target/release/image-webp-converter /usr/local/bin
```

## Usage

```bash	
$ image-webp-converter

Usage: image-webp-converter [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>      
  -s, --scale <SCALE>      [default: 1]
  -q, --quality <QUALITY>  [default: 100]
  -o, --output <OUTPUT>    
  -h, --help               Print help
  -V, --version            Print version
```
