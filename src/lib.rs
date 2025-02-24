use eframe::egui::ColorImage;
use image::{DynamicImage, ImageBuffer, ImageError, Pixel};
use integer_sqrt::IntegerSquareRoot;
use std::ops::Deref;

pub fn load_img_from_buffer<P: Pixel, Container>(
    img_buffer: ImageBuffer<P, Container>,
) -> Result<ColorImage, ImageError>
where
    P: Pixel<Subpixel = u8>,
    Container: Deref<Target = [P::Subpixel]> + std::convert::AsRef<[u8]>,
{
    let pixels = img_buffer.as_flat_samples();
    let len = img_buffer.len().integer_sqrt() / 2;

    let size: [usize; 2] = [len, len];
    Ok(ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
}

use clipboard_rs::{common::RustImage, Clipboard, ClipboardContext, RustImageData};
use image::Rgba;

pub fn copy_image(img: ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let ctx = ClipboardContext::new().unwrap();

    let types = ctx.available_formats().unwrap();
    println!("{:?}", types);

    let img: DynamicImage = DynamicImage::from(img);
    let img: RustImageData = RustImageData::from_dynamic_image(img);

    ctx.set_image(img).expect("Copy image in clipboard.");
}
