use eframe::egui;

/// Aplica o tema cyberpunk à aplicação.
/// Centraliza todas as configurações de estilo visual.
pub fn apply_cyberpunk_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    // Modo escuro com cores neon
    style.visuals.dark_mode = true;
    style.visuals.override_text_color = Some(egui::Color32::from_rgb(200, 200, 255));
    style.visuals.panel_fill = egui::Color32::from_rgb(10, 10, 25);
    style.visuals.window_fill = egui::Color32::from_rgba_premultiplied(15, 15, 35, 240);
    style.visuals.extreme_bg_color = egui::Color32::from_rgb(5, 5, 15);
    
    // Widgets com efeito neon
    configure_widget_styles(&mut style);
    
    // Efeitos adicionais
    style.visuals.selection.bg_fill = egui::Color32::from_rgba_premultiplied(138, 43, 226, 100);
    style.visuals.window_shadow.color = egui::Color32::from_rgba_premultiplied(138, 43, 226, 50);
    
    ctx.set_style(style);
}

fn configure_widget_styles(style: &mut egui::Style) {
    style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(20, 20, 40);
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 30, 50);
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(50, 50, 80);
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(80, 40, 120);
    
    style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(138, 43, 226));
    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 255, 255));
    style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 0, 255));
    style.visuals.widgets.active.bg_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 255, 0));
}

/// Cria o ícone da aplicação.
pub fn create_icon() -> egui::IconData {
    const ICON_SIZE: usize = 32;
    let mut rgba = vec![0u8; ICON_SIZE * ICON_SIZE * 4];
    
    for (i, pixel) in rgba.chunks_mut(4).enumerate() {
        let x = i % ICON_SIZE;
        let y = i / ICON_SIZE;
        
        let center = ICON_SIZE / 2;
        let dx = (x as i32 - center as i32).abs();
        let dy = (y as i32 - center as i32).abs();
        let dist = ((dx * dx + dy * dy) as f32).sqrt();
        
        if dist < 14.0 && dist > 10.0 {
            // Anel externo roxo neon
            pixel[0] = 138;
            pixel[1] = 43;
            pixel[2] = 226;
            pixel[3] = 255;
        } else if dist < 10.0 {
            // Centro cyan neon
            pixel[0] = 0;
            pixel[1] = 255;
            pixel[2] = 255;
            pixel[3] = 255;
        }
    }
    
    egui::IconData {
        rgba,
        width: ICON_SIZE as u32,
        height: ICON_SIZE as u32,
    }
}

/// Configura as fontes customizadas.
pub fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    fonts.families.insert(
        egui::FontFamily::Proportional,
        vec!["Hack".to_owned(), "Ubuntu-Light".to_owned()],
    );
    
    ctx.set_fonts(fonts);
}
