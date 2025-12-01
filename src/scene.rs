use eframe::{
    CreationContext,
    egui::{ColorImage, TextureHandle},
};

pub struct Scene {
    pub available_images: Vec<TextureHandle>,
    selected_image: usize,
    /// # of colors in transformed images
    n_colors: u32,

    // transformed images
    uncert_image: TextureHandle,
    popula_image: TextureHandle,
    kmeans_image: TextureHandle,
}

impl Scene {
    pub fn new(cc: &CreationContext) -> Self {
        let orig_texture = Self::open_image(cc, "assets/maklo.jpg");
        let available_images = vec![
            orig_texture.clone(),
            Self::open_image(cc, "assets/mandel_normalmap.jpg"),
        ];

        Self {
            available_images,
            selected_image: 0,
            n_colors: 10,
            uncert_image: Self::reduce_uncertainty(orig_texture.clone()),
            popula_image: Self::reduce_uncertainty(orig_texture.clone()),
            kmeans_image: Self::reduce_uncertainty(orig_texture.clone()),
        }
    }

    fn open_image(cc: &CreationContext, path: &str) -> TextureHandle {
        let img = image::open(path).expect("failed to load image");
        let rgba = img.to_rgba8();
        let size = [img.width() as usize, img.height() as usize];
        let color_image = ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
        let texture = cc
            .egui_ctx
            .load_texture("orig_image", color_image, Default::default());
        texture
    }

    pub fn select_main_image(&mut self, index: usize) {
        self.selected_image = index;
        self.uncert_image = Self::reduce_uncertainty(self.main_image().clone());
        self.popula_image = Self::reduce_uncertainty(self.main_image().clone());
        self.kmeans_image = Self::reduce_uncertainty(self.main_image().clone());
    }

    pub fn main_image(&self) -> &TextureHandle {
        &self.available_images[self.selected_image]
    }

    pub fn n_colors_mut(&mut self) -> &mut u32 {
        &mut self.n_colors
    }

    fn reduce_uncertainty(img: TextureHandle) -> TextureHandle {
        img
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
