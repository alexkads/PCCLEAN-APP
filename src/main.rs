mod cleaner;
mod ui;
mod scanner;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(create_icon()),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "PCCLEAN - Cyberpunk Disk Cleaner",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(ui::PCCleanApp::new(cc)))
        }),
    )
}

fn create_icon() -> egui::IconData {
    // Ícone simples 32x32 (roxo neon)
    let icon_size = 32;
    let mut rgba = vec![0u8; icon_size * icon_size * 4];
    
    for (i, pixel) in rgba.chunks_mut(4).enumerate() {
        let x = i % icon_size;
        let y = i / icon_size;
        
        // Criar um padrão circular neon
        let center = icon_size / 2;
        let dx = (x as i32 - center as i32).abs();
        let dy = (y as i32 - center as i32).abs();
        let dist = ((dx * dx + dy * dy) as f32).sqrt();
        
        if dist < 14.0 && dist > 10.0 {
            pixel[0] = 138; // R
            pixel[1] = 43;  // G
            pixel[2] = 226; // B (roxo neon)
            pixel[3] = 255; // A
        } else if dist < 10.0 {
            pixel[0] = 0;
            pixel[1] = 255;
            pixel[2] = 255; // Cyan neon
            pixel[3] = 255;
        }
    }
    
    egui::IconData {
        rgba,
        width: icon_size as u32,
        height: icon_size as u32,
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    // Usar fonte monoespaçada para estilo cyberpunk
    fonts.families.insert(
        egui::FontFamily::Proportional,
        vec!["Hack".to_owned(), "Ubuntu-Light".to_owned()],
    );
    
    ctx.set_fonts(fonts);
}
