mod app;
mod camera;
mod draw;
mod model;

use eframe::egui;

fn main() -> eframe::Result {
    // Initialization menu window
    let onnx_files = match model::read_model_contents() {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Error reading model contents: {}", e);
            Vec::new()
        }
    };
    println!("Found ONNX files: {:?}", onnx_files);

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "BrushQuadX VisionOTG",
        options,
        Box::new(move |cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(app::App {
                onnx_files,
                ..Default::default()
            }))
        }),
    )
}
