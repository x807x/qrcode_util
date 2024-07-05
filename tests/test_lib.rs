#[cfg(test)]
mod tests {
    use egui_learn::*;
    use image::{ImageBuffer, Luma, Pixel, Rgba};

    #[test]
    fn test_create_qrcode() {
        let data = "This is the data of the qrcode!";
        let img = create_qrcode(data).unwrap();

        let width: u32 = img.width().try_into().unwrap();
        let height: u32 = img.height().try_into().unwrap();
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
}
