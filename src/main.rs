#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::ColorsApp;

mod app;
mod scene;
mod reducer;
mod uncert_reducer;
mod popularity_reducer;

fn main() -> eframe::Result {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = native_options.viewport.with_maximized(true);
    eframe::run_native(
        "Color reduction",
        native_options,
        Box::new(|cc| Ok(Box::new(ColorsApp::new(cc)))),
    )
}
