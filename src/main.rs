pub mod functions;

use clap::Parser;
use functions::*;
use image::GenericImageView;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value_t = 1f32)]
    scale: f32,

    #[arg(short, long, default_value_t = 100f32)]
    quality: f32,

    #[arg(short, long)]
    output: String,
}

fn main() -> Result<(), ImageError> {
    let args = Args::parse();

    let mut image = load_image(&args.input).map_err(|_| ImageError::ImageNotLoaded)?;

    if args.scale != 1f32 {
        let (current_width, current_height) = image.dimensions();
        let width = current_width as f32 * args.scale;
        let height = current_height as f32 * args.scale;
        image = resize_image(image, width as u32, height as u32);
    }

    let webp = convert_to_webp(image, args.quality)?;

    save_webp(&webp, &args.output)
}
