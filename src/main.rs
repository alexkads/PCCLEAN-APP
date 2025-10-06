// Camadas da aplicação seguindo DDD e Clean Architecture
mod domain;
mod application;
mod infrastructure;
mod presentation;
mod shared;

use eframe::egui;
use presentation::PCCleanApp;

/// Ponto de entrada da aplicação.
/// Configura e inicializa a interface gráfica.
fn main() -> eframe::Result<()> {
    let options = create_native_options();

    eframe::run_native(
        "PCCLEAN - Ultra Modern Disk Cleaner",
        options,
        Box::new(|cc| {
            Ok(Box::new(PCCleanApp::new(cc)))
        }),
    )
}

fn create_native_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 900.0])
            .with_min_inner_size([900.0, 650.0]),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    }
}
