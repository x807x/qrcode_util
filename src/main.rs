use eframe::egui::{self, TextureHandle};
use image::{ImageBuffer, Rgba};
use qrcode::QrCode;
use std::option::Option;
use integer_sqrt::IntegerSquareRoot;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My Rust GUI App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<QRCodeApp>::default()
        }),
    )
}

struct QRCodeApp {
    data: String,
    texture: Option<TextureHandle>,
}

impl Default for QRCodeApp {
    fn default() -> Self {
        Self {
            data: "World".to_owned(),
            texture: None,
        }
    }
}

fn load_qrcode_from_text(text: &str) -> Result<egui::ColorImage, image::ImageError> {
    let code = QrCode::new(text).unwrap();
    let image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = code.render::<Rgba<u8>>().build();
    let pixels = image_buffer.as_flat_samples();
    let len = image_buffer.len().integer_sqrt()/2;
    
    let size: [usize; 2] = [len, len];
    println!("{:#?}", size);
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

impl eframe::App for QRCodeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, Rust QRcode!");
            ui.horizontal(|ui| {
                ui.label("Your QRcode content");
                ui.text_edit_multiline(&mut self.data);
            });
            let texture = self.texture.insert({
                ui.ctx().load_texture(
                    "QRcode",
                    load_qrcode_from_text(self.data.as_str()).unwrap(),
                    egui::TextureOptions::default(),
                )
            });
            ui.image((texture.id(), texture.size_vec2()));
        });
    }
}
