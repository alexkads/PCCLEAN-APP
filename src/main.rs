// Camadas da aplicação seguindo DDD e Clean Architecture
mod domain;
mod application;
mod infrastructure;
mod presentation;
mod shared;

use eframe::egui;
use presentation::{PCCleanApp, create_icon, setup_custom_fonts};

/// Ponto de entrada da aplicação.
/// Configura e inicializa a interface gráfica.
fn main() -> eframe::Result<()> {
    let options = create_native_options();

    eframe::run_native(
        "PCCLEAN - Cyberpunk Disk Cleaner DDD",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(PCCleanApp::new(cc)))
        }),
    )
}

fn create_native_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(create_icon()),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    }
}
