use eframe::egui::ColorImage;
use image::{ImageBuffer, Rgba};
use integer_sqrt::IntegerSquareRoot;
use qrcode::QrCode;

pub fn create_qrcode(text: &str) -> Result<ColorImage, image::ImageError> {
    let code = QrCode::new(text).unwrap();
    let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = code.render::<Rgba<u8>>().build();
    let pixels = img_buffer.as_flat_samples();
    let len = img_buffer.len().integer_sqrt() / 2;

    let size: [usize; 2] = [len, len];
    Ok(ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
