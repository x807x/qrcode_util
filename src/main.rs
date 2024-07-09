use eframe::egui::{self, TextureHandle};
use image::{ImageBuffer, Rgba};
use qrcode_util::*;
use std::option::Option;
use qrcode::QrCode;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "My Rust QRCode App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<QRCodeApp>::default())
        }),
    )
}

struct QRCodeApp {
    previous_data: String,
    data: String,
    texture: Option<TextureHandle>,
    img_buffer: Option<ImageBuffer<Rgba<u8>, Vec<u8>>>,
}

impl Default for QRCodeApp {
    fn default() -> Self {
        Self {
            previous_data: "".to_owned(),
            data: "World".to_owned(),
            texture: None,
            img_buffer: None,
        }
    }
}

impl eframe::App for QRCodeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello, Rust QRCode!");
            ui.horizontal(|ui| {
                ui.label("Your QRCode content");
                ui.text_edit_multiline(&mut self.data);
            });
            let texture: &mut TextureHandle = if self.previous_data != self.data {
                self.data.clone_into(&mut self.previous_data);
                let code = QrCode::new(&self.data).unwrap();
                self.img_buffer = Some(code.render::<Rgba<u8>>().build());
                
                self.texture.insert({
                    ui.ctx().load_texture(
                        "QRCode",
                        load_img_from_buffer(self.img_buffer.to_owned().unwrap()).unwrap(),
                        egui::TextureOptions::default(),
                    )
                })
            } else {
                self.texture.as_mut().expect("Expect Some in Texture")
            };
            ui.menu_image_button((texture.id(), texture.size_vec2()), |ui| {
                if ui.button("copy image").clicked() {
                    copy_image(self.img_buffer.to_owned().unwrap());
                }
                if ui.button("save image").clicked() {
                    todo!("Save image");
                }
                if ui.button("❌ Close menu").clicked() {
                    ui.close_menu();
                }
            });
        });
    }
}
