#[cfg(test)]
mod tests {
    use qrcode::QrCode;
    use image::{Luma, ImageBuffer, Rgba, Pixel};
    use qrcode_util::{copy_image, load_img_from_buffer};

     #[test]
     fn test_load_img_from_buffer() {
        let data = "adsfasdfa";
        let code = QrCode::new(&data).unwrap();
        let img_buffer = code.render::<Luma<u8>>().build();
        
        let img = load_img_from_buffer(img_buffer)
            .expect("Failed to load ColorImage from ImageBuffer");


        let width: u32 = img.width().try_into().unwrap();
        let height: u32 = img.height().try_into().unwrap();
        assert_eq!(width, height);
        let mut img_buffer: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        for pixel in img.pixels {
            if x == width {
                y += 1;
                x = 0;
            }
            let rgba: Rgba<u8> = Rgba::from([pixel.r(), pixel.g(), pixel.b(), pixel.a()]);
            img_buffer.put_pixel(x, y, rgba.to_luma());
            x += 1;
        }
        let mut img = rqrr::PreparedImage::prepare(img_buffer);
        let grids = img.detect_grids();
        let (_, content) = grids[0].decode().unwrap();
        println!("data: {:?}, content: {:?}", data, content);
        assert_eq!(data, content);
     }


     use clipboard_rs::{Clipboard, ClipboardContext, RustImageData};
     use std::error::Error;

     #[test]
     fn test_copy_image() {
        let data: &str= "Test Data";
        let code: QrCode = QrCode::new(data).unwrap();
        let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = code.render::<Rgba<u8>>().build();
        
        let ctx = ClipboardContext::new().unwrap();
        ctx.clear().unwrap();
        assert!(!ctx.has(clipboard_rs::ContentFormat::Image));

        copy_image(img_buffer);

        let types: Vec<String> = ctx.available_formats().unwrap_or(Vec::new());
        assert!(types.contains(&"PNG".to_owned()));

        let img: Result<RustImageData, Box<dyn Error + Send + Sync>> = ctx.get_image();

        assert!(img.is_ok());
     }
}