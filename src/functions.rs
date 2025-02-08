#[derive(Debug)]
pub enum ImageError {
    ImageNotLoaded,
    CanNotEncodeImage,
    QualityIsOutOfRange,
    CanNotSave,
}

pub fn load_image(path: &str) -> Result<image::DynamicImage, image::ImageError> {
    image::open(path)
}

pub fn resize_image(image: image::DynamicImage, width: u32, height: u32) -> image::DynamicImage {
    image.resize(width, height, image::imageops::FilterType::Lanczos3)
}

pub fn convert_to_webp(
    image: image::DynamicImage,
    quality: f32,
) -> Result<webp::WebPMemory, ImageError> {
    let enc = webp::Encoder::from_image(&image).map_err(|_| ImageError::CanNotEncodeImage)?;

    if quality > 100f32 || quality < 0f32 {
        return Err(ImageError::QualityIsOutOfRange);
    }

    Ok(enc.encode(quality))
}

pub fn save_webp(webp: &webp::WebPMemory, path: &str) -> Result<(), ImageError> {
    std::fs::write(&path, webp.to_vec()).map_err(|_| ImageError::CanNotSave)
}
