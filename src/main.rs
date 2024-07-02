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

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    name: String,
    texture: Option<TextureHandle>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "World".to_owned(),
            texture: None,
        }
    }
}

fn load_qrcode_from_text(text: &str) -> Result<egui::ColorImage, image::ImageError> {
    let code = QrCode::new(text).unwrap();
    let image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = code.render::<Rgba<u8>>().build();
    image_buffer.save("image.png").unwrap();
    let pixels = image_buffer.as_flat_samples();
    let len = image_buffer.len().integer_sqrt()/2;
    
    let size: [usize; 2] = [len, len];
    println!("{:#?}", size);
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, Rust GUI!");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.label(format!("Hello, {}!", self.name));
            let texture = self
                .texture
                .get_or_insert(egui::Context::default().load_texture(
                    "QRcode",
                    load_qrcode_from_text(self.name.as_str()).unwrap(),
                    egui::TextureOptions::default(),
                ));
            ui.image(&texture.to_owned());
        });
    }
}
