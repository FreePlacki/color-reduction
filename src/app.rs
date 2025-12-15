use eframe::{
    CreationContext,
    egui::{self, CentralPanel, Sense, SidePanel, Slider, TopBottomPanel, Ui, Visuals},
};

use crate::{scene::Scene, uncert_reducer::DiffusionMatrix};

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
        for (i, img) in self.scene.available_images.iter().enumerate() {
            if ui
                .add(
                    egui::Image::new(&img.texture)
                        .max_size(ui.available_size())
                        .sense(Sense::click()),
                )
                .clicked()
            {
                selected_idx = Some(i);
            }
        }
        if let Some(i) = selected_idx {
            self.scene.select_main_image(i, ui.ctx());
        }
    }
}

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
                let mut n = self.scene.n_colors();
                ui.add(Slider::new(&mut n, 1..=100).logarithmic(true));
                self.scene.update_n_colors(n, ctx);
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
                            let mut m = self.scene.diffusion_matrix();
                            ui.radio_value(
                                &mut m,
                                DiffusionMatrix::FloydSteinberg,
                                "Floyd-Steinberg",
                            );
                            ui.radio_value(&mut m, DiffusionMatrix::Burkes, "Burkes");
                            ui.radio_value(&mut m, DiffusionMatrix::Stucky, "Stucky");
                            self.scene.update_diffusion_matrix(m);
                        });

                        ui.add(
                            egui::Image::new(self.scene.uncert_image())
                                .max_size(ui.available_size()),
                        );
                    });

                    cols[1].group(|ui| {
                        ui.heading("Reduced using popularity algorithm");
                        ui.separator();
                        ui.label("");
                        ui.add(
                            egui::Image::new(self.scene.popula_image())
                                .max_size(ui.available_size()),
                        );
                    });

                    cols[2].group(|ui| {
                        ui.heading("Reduced using k-means algorithm");
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label("Epsilon value: ");
                            let mut eps = self.scene.kmeans_eps();
                            ui.add(egui::Slider::new(&mut eps, 10.0..=50.0).logarithmic(true));
                            self.scene.update_kmeans_eps(eps);
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
                ui.label("Choose image");
                ui.vertical(|ui| {
                    self.image_row(ui);
                });
            });

        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.group(|ui| {
                    ui.label("Original image");
                    ui.add(egui::Image::new(self.scene.main_image()).max_size(ui.available_size()));
                });
            });
        });

        self.scene.poll_results(ctx);
        ctx.request_repaint();
    }
}
