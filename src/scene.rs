use eframe::{
    CreationContext,
    egui::{self, ColorImage, TextureHandle},
};

use crate::{
    popularity_reducer::PopularityReducer,
    reducer::Reducer,
    uncert_reducer::{DiffusionMatrix, UncertReducer},
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

    popula_reducer: Option<PopularityReducer>,
}

impl Scene {
    pub fn new(cc: &CreationContext) -> Self {
        let orig_texture = Self::open_image(cc, "assets/maklo.jpg");
        let available_images = vec![
            orig_texture.clone(),
            Self::open_image(cc, "assets/mandel_normalmap.jpg"),
        ];

        let mut s = Self {
            available_images,
            selected_image: 0,
            n_colors: 10,
            diffusion_matrix: Default::default(),
            uncert_image: orig_texture.texture.clone(),
            popula_image: orig_texture.texture.clone(),
            kmeans_image: orig_texture.texture,
            popula_reducer: None,
        };
        s.compute_all(&cc.egui_ctx, true);

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
        self.compute_all(ctx, true);
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
        self.compute_all(ctx, false);
    }

    pub fn diffusion_matrix(&self) -> DiffusionMatrix {
        self.diffusion_matrix
    }

    pub fn update_diffusion_matrix(&mut self, matrix: DiffusionMatrix, ctx: &egui::Context) {
        if self.diffusion_matrix == matrix {
            return;
        }

        self.diffusion_matrix = matrix;
        self.compute_uncert(ctx);
    }

    fn compute_all(&mut self, ctx: &egui::Context, image_changed: bool) {
        self.compute_uncert(ctx);
        if image_changed {
            self.popula_reducer = Some(PopularityReducer::new(
                &self.available_images[self.selected_image].rgba,
            ));
        }
        self.compute_popula(ctx);
    }

    fn compute_uncert(&mut self, ctx: &egui::Context) {
        let img = &self.available_images[self.selected_image];
        let (w, h) = {
            let s = img.texture.size();
            (s[0], s[1])
        };

        let uncert = UncertReducer::with_uniform_palette(self.diffusion_matrix);
        let out = uncert.reduce(&img.rgba, w, h, self.n_colors);

        let col = eframe::epaint::ColorImage::from_rgba_unmultiplied(img.texture.size(), &out);
        self.uncert_image = ctx.load_texture("uncert-computed", col, Default::default());
    }

    fn compute_popula(&mut self, ctx: &egui::Context) {
        let img = &self.available_images[self.selected_image];
        let (w, h) = {
            let s = img.texture.size();
            (s[0], s[1])
        };

        let out = self
            .popula_reducer
            .as_ref()
            .unwrap()
            .reduce(&img.rgba, w, h, self.n_colors);

        let col = eframe::epaint::ColorImage::from_rgba_unmultiplied(img.texture.size(), &out);
        self.popula_image = ctx.load_texture("popula-computed", col, Default::default());
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
}
