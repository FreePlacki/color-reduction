use eframe::{
    CreationContext,
    egui::{ColorImage, TextureHandle},
};

pub struct Scene {
    pub orig_image: TextureHandle,
}

impl Scene {
    pub fn new(cc: &CreationContext) -> Self {
        let img = image::open("assets/maklo.jpg").expect("failed to load image");
        let rgba = img.to_rgba8();
        let size = [img.width() as usize, img.height() as usize];
        let color_image = ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
        let texture = cc
            .egui_ctx
            .load_texture("orig_image", color_image, Default::default());

        Self {
            orig_image: texture,
        }
    }
}
