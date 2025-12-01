use eframe::{
    CreationContext,
    egui::{self, CentralPanel, Sense, SidePanel, Slider, TopBottomPanel, Ui, Visuals},
};

use crate::scene::Scene;

pub struct ColorsApp {
    scene: Scene,
}

impl ColorsApp {
    pub fn new(cc: &CreationContext) -> Self {
        let scene = Scene::new(cc);
        Self { scene }
    }

    fn image_row(&mut self, ui: &mut Ui) {
        let mut selected_idx = None;
        for (i, texture) in self.scene.available_images.iter().enumerate() {
            if ui
                .add(
                    egui::Image::new(texture)
                        .max_size(ui.available_size())
                        .sense(Sense::click()),
                )
                .clicked()
            {
                selected_idx = Some(i);
            }
        }
        if let Some(i) = selected_idx {
            self.scene.select_main_image(i);
        }
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
                ui.label("Colors:");
                ui.add(Slider::new(self.scene.n_colors_mut(), 1..=50));
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
                        ui.horizontal(|ui| {
                            ui.label("Select filter matrix:");
                            ui.radio_value(&mut 0, 0, "Floyd-Steinberg");
                            ui.radio_value(&mut 0, 1, "Burkes");
                            ui.radio_value(&mut 0, 2, "Stucky");
                        });

                        ui.add(
                            egui::Image::new(self.scene.uncert_image())
                                .max_size(ui.available_size()),
                        );
                    });

                    cols[1].group(|ui| {
                        ui.heading("Reduced using popularity algorithm");
                        ui.separator();
                        ui.add(
                            egui::Image::new(self.scene.popula_image())
                                .max_size(ui.available_size()),
                        );
                    });

                    cols[2].group(|ui| {
                        ui.heading("Reduced using k-means algorithm");
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label("Epsilon value: 11");
                            ui.add(egui::Slider::new(&mut 11, 1..=50));
                        });
                        ui.add(
                            egui::Image::new(self.scene.kmeans_image())
                                .max_size(ui.available_size()),
                        );
                    });
                });
            });

        SidePanel::right("thumbnails")
            .resizable(true)
            .min_width(200.0)
            .max_width(500.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    self.image_row(ui);
                });
            });

        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.label("Main image preview area");
                    ui.add(egui::Image::new(self.scene.main_image()).max_size(ui.available_size()));
                });
            });
        });
    }
}
