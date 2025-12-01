use eframe::{CreationContext, egui::{self, CentralPanel, SidePanel, TopBottomPanel, Ui, Visuals}};

use crate::scene::Scene;

pub struct ColorsApp {
    scene: Scene,
}

impl ColorsApp {
    pub fn new(cc: &CreationContext) -> Self {
        let scene = Scene::new(cc);
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

        TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
            });
        });

        TopBottomPanel::bottom("processed")
            .min_height(300.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.columns(3, |cols| {
                    cols[0].group(|ui| {
                        ui.heading("Reduced using propagation of uncertainty");
                        ui.separator();
                        ui.label("Select filter matrix:");
                        ui.radio_value(&mut 0, 0, "Floyd-Steinberg");
                        ui.radio_value(&mut 0, 1, "Burkes");
                        ui.radio_value(&mut 0, 2, "Stucky");

                        ui.add_space(5.0);
                        ui.label("Preview");
                        ui.allocate_space(ui.available_size());
                    });

                    cols[1].group(|ui| {
                        ui.heading("Reduced using popularity algorithm");
                        ui.separator();
                        ui.label("Preview");
                        ui.allocate_space(ui.available_size());
                    });

                    cols[2].group(|ui| {
                        ui.heading("Reduced using k-means algorithm");
                        ui.separator();
                        ui.label("Epsilon value: 11");
                        ui.add(egui::Slider::new(&mut 11, 1..=50));
                        ui.label("Preview");
                        ui.allocate_space(ui.available_size());
                    });
                });
            });

        SidePanel::right("thumbnails")
            .resizable(false)
            .min_width(200.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    image_row(ui);
                    image_row(ui);
                    image_row(ui);
                });
            });

        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.label("Main image preview area");
                    ui.add(egui::Image::new(&self.scene.orig_image).max_size(ui.available_size()));
                    // ui.image(&self.scene.orig_image);
                });
            });
        });
    }
}

fn image_row(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.group(|ui| {
            ui.label("Thumb A");
            ui.allocate_space([80.0, 60.0].into());
        });
        ui.group(|ui| {
            ui.label("Thumb B");
            ui.allocate_space([80.0, 60.0].into());
        });
    });
}
