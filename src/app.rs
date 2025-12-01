use eframe::egui::{self, Visuals};

use crate::scene::Scene;

pub struct ColorsApp {
    scene: Scene,
}

impl ColorsApp {
    pub fn new() -> Self {
        let scene = Scene::default();
        Self { scene }
    }
}

impl ColorsApp {}

impl eframe::App for ColorsApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        ctx.style_mut(|style| {
            style.wrap_mode = Some(egui::TextWrapMode::Extend);
            style.visuals = Visuals::dark();
        });

        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
            });
        });
    }
}
