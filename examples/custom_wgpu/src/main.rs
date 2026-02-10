#![expect(clippy::unwrap_used)] // TODO(emilk): avoid unwraps
use custom_wgpu::Custom3d;
use eframe::egui;

fn main() {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 1024.0])
            .with_drag_and_drop(true),

        renderer: eframe::Renderer::Wgpu,

        ..Default::default()
    };

    let result = eframe::run_native(
        "egui demo app",
        options,
        Box::new(|cc| Ok(Box::new(Custom3d::new(cc)))),
    );
    result.unwrap();
}
