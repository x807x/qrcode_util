use eframe::egui::{self, TextureHandle};
use egui_learn::*;
use std::option::Option;

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
}

impl Default for QRCodeApp {
    fn default() -> Self {
        Self {
            previous_data: "".to_owned(),
            data: "World".to_owned(),
            texture: None,
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
            let texture: &mut TextureHandle;
            if self.previous_data != self.data {
                self.previous_data = self.data.to_owned();
                texture = self.texture.insert({
                    ui.ctx().load_texture(
                        "QRCode",
                        create_qrcode(self.data.as_str()).unwrap(),
                        egui::TextureOptions::default(),
                    )
                });
            } else {
                texture = self.texture.as_mut().expect("Expect Some in Texture");
            }
            ui.image((texture.id(), texture.size_vec2()));
        });
    }
}
