use eframe::{
    CreationContext,
    egui::{self, ColorImage, TextureHandle},
};

use crate::{
    kmeans_reducer::KMeansReducer,
    popularity_reducer::PopularityReducer,
    uncert_reducer::{DiffusionMatrix, UncertReducer},
    worker::{ComputeRequest, ComputeWorker},
};

#[derive(Clone)]
pub struct LoadedImage {
    pub texture: TextureHandle,
    rgba: Vec<u8>,
}

pub struct Scene {
    pub available_images: Vec<LoadedImage>,
    selected_image: usize,
    /// # of colors in transformed images
    n_colors: usize,
    /// used for propagation of uncertainty
    diffusion_matrix: DiffusionMatrix,

    // transformed images
    uncert_image: TextureHandle,
    popula_image: TextureHandle,
    kmeans_image: TextureHandle,

    /// kmeans stop condition parameter
    kmeans_eps: f32,

    worker: ComputeWorker,
}

impl Scene {
    pub fn new(cc: &CreationContext) -> Self {
        let orig_texture = Self::open_image(cc, "assets/maklo.jpg");
        let available_images = vec![
            orig_texture.clone(),
            Self::open_image(cc, "assets/mandel_normalmap.jpg"),
            Self::open_image(cc, "assets/mini.jpg"),
            Self::open_image(cc, "assets/wallpaper.jpg"),
        ];

        let mut s = Self {
            available_images,
            selected_image: 0,
            n_colors: 10,
            diffusion_matrix: Default::default(),
            uncert_image: orig_texture.texture.clone(),
            popula_image: orig_texture.texture.clone(),
            kmeans_image: orig_texture.texture,
            kmeans_eps: 50.0,
            worker: ComputeWorker::spawn(),
        };
        s.compute_all(&cc.egui_ctx);

        s
    }

    fn open_image(cc: &CreationContext, path: &str) -> LoadedImage {
        let img = image::open(path).expect("failed to load image");
        let rgba = img.to_rgba8().to_vec();
        let size = [img.width() as usize, img.height() as usize];
        let color_image = ColorImage::from_rgba_unmultiplied(size, &rgba);
        let texture = cc
            .egui_ctx
            .load_texture("orig_image", color_image, Default::default());
        LoadedImage { texture, rgba }
    }

    pub fn select_main_image(&mut self, index: usize, ctx: &egui::Context) {
        self.selected_image = index;
        self.compute_all(ctx);
    }

    pub fn main_image(&self) -> &TextureHandle {
        &self.available_images[self.selected_image].texture
    }

    pub fn n_colors(&self) -> usize {
        self.n_colors
    }

    pub fn update_n_colors(&mut self, n: usize, ctx: &egui::Context) {
        if n == self.n_colors {
            return;
        }

        self.n_colors = n;
        self.compute_all(ctx);
    }

    pub fn diffusion_matrix(&self) -> DiffusionMatrix {
        self.diffusion_matrix
    }

    pub fn update_diffusion_matrix(&mut self, matrix: DiffusionMatrix) {
        if self.diffusion_matrix == matrix {
            return;
        }

        self.diffusion_matrix = matrix;
        let req = self.uncert_req();
        let _ = self.worker.tx.send(req);
    }

    pub fn poll_results(&mut self, ctx: &egui::Context) {
        while let Ok(res) = self.worker.rx.try_recv() {
            let size = [res.width, res.height];

            if let Some(uncert) = res.uncert {
                let col = eframe::epaint::ColorImage::from_rgba_unmultiplied(size, &uncert);
                self.uncert_image = ctx.load_texture("uncert-computed", col, Default::default());
            }
            if let Some(popula) = res.popula {
                let col = eframe::epaint::ColorImage::from_rgba_unmultiplied(size, &popula);
                self.popula_image = ctx.load_texture("popula-computed", col, Default::default());
            }
            if let Some(kmeans) = res.kmeans {
                let col = eframe::epaint::ColorImage::from_rgba_unmultiplied(size, &kmeans);
                self.kmeans_image = ctx.load_texture("kmeans-computed", col, Default::default());
            }

            ctx.request_repaint();
        }
    }

    fn compute_all(&mut self, ctx: &egui::Context) {
        let mut req = self.uncert_req();
        req.popula_reducer = self.popula_req().popula_reducer;
        req.kmeans_reducer = self.kmeans_req().kmeans_reducer;

        let _ = self.worker.tx.send(req);
        ctx.request_repaint();
    }

    fn uncert_req(&mut self) -> ComputeRequest {
        let img = &self.available_images[self.selected_image];
        let (width, height) = {
            let s = img.texture.size();
            (s[0], s[1])
        };

        let reducer = UncertReducer::with_uniform_palette(self.diffusion_matrix);

        ComputeRequest {
            img: img.rgba.clone(),
            n_colors: self.n_colors,
            width,
            height,
            uncert_reducer: Some(reducer),
            popula_reducer: None,
            kmeans_reducer: None,
        }
    }

    fn popula_req(&mut self) -> ComputeRequest {
        let img = &self.available_images[self.selected_image];
        let (width, height) = {
            let s = img.texture.size();
            (s[0], s[1])
        };

        let reducer = PopularityReducer::new(&self.available_images[self.selected_image].rgba);

        ComputeRequest {
            img: img.rgba.clone(),
            n_colors: self.n_colors,
            width,
            height,
            uncert_reducer: None,
            popula_reducer: Some(reducer),
            kmeans_reducer: None,
        }
    }

    fn kmeans_req(&mut self) -> ComputeRequest {
        let img = &self.available_images[self.selected_image];
        let (width, height) = {
            let s = img.texture.size();
            (s[0], s[1])
        };

        let reducer = KMeansReducer::new(&img.rgba, self.kmeans_eps);

        ComputeRequest {
            img: img.rgba.clone(),
            n_colors: self.n_colors,
            width,
            height,
            uncert_reducer: None,
            popula_reducer: None,
            kmeans_reducer: Some(reducer),
        }
    }

    pub fn uncert_image(&self) -> &TextureHandle {
        &self.uncert_image
    }

    pub fn popula_image(&self) -> &TextureHandle {
        &self.popula_image
    }

    pub fn kmeans_image(&self) -> &TextureHandle {
        &self.kmeans_image
    }

    pub fn kmeans_eps(&self) -> f32 {
        self.kmeans_eps
    }

    pub fn update_kmeans_eps(&mut self, eps: f32) {
        if eps == self.kmeans_eps {
            return;
        }
        self.kmeans_eps = eps;
        let req = self.kmeans_req();
        let _ = self.worker.tx.send(req);
    }
}
