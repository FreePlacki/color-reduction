use eframe::{
    CreationContext,
    egui::{ColorImage, TextureHandle},
};

pub struct Scene {
    pub available_images: Vec<TextureHandle>,
    selected_image: usize,
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
    }

    pub fn main_image(&self) -> &TextureHandle {
        &self.available_images[self.selected_image]
    }
}
