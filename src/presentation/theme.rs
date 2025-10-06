use eframe::egui;

// Paleta de cores HUD Sci-Fi
pub struct HudColors;

impl HudColors {
    // Cores principais HUD
    pub const CYAN_PRIMARY: egui::Color32 = egui::Color32::from_rgb(0, 255, 255);
    pub const CYAN_DARK: egui::Color32 = egui::Color32::from_rgb(0, 180, 200);
    pub const CYAN_GLOW: egui::Color32 = egui::Color32::from_rgb(0, 220, 240);
    
    pub const RED_ALERT: egui::Color32 = egui::Color32::from_rgb(255, 50, 80);
    pub const ORANGE_WARNING: egui::Color32 = egui::Color32::from_rgb(255, 150, 50);
    pub const GREEN_OK: egui::Color32 = egui::Color32::from_rgb(0, 255, 150);
    
    // Backgrounds tech
    pub const BG_DARK: egui::Color32 = egui::Color32::from_rgb(5, 15, 25);
    pub const BG_PANEL: egui::Color32 = egui::Color32::from_rgb(10, 25, 35);
    pub const BG_ELEVATED: egui::Color32 = egui::Color32::from_rgb(15, 35, 50);
    
    // Textos
    pub const TEXT_PRIMARY: egui::Color32 = egui::Color32::from_rgb(200, 240, 255);
    pub const TEXT_DIM: egui::Color32 = egui::Color32::from_rgb(100, 150, 170);
}

/// Aplica tema HUD Sci-Fi à aplicação.
pub fn apply_cyberpunk_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    // Background tech escuro
    style.visuals.dark_mode = true;
    style.visuals.override_text_color = Some(HudColors::TEXT_PRIMARY);
    style.visuals.panel_fill = HudColors::BG_DARK;
    style.visuals.window_fill = HudColors::BG_PANEL;
    style.visuals.extreme_bg_color = HudColors::BG_DARK;
    
    // Widgets estilo HUD
    configure_widget_styles(&mut style);
    
    // Efeitos visuais
    style.visuals.selection.bg_fill = egui::Color32::from_rgba_premultiplied(0, 255, 255, 60);
    style.visuals.window_shadow.color = egui::Color32::from_rgba_premultiplied(0, 255, 255, 100);
    style.visuals.window_rounding = egui::Rounding::same(0.0); // Cantos retos tech
    
    // Espaçamento
    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(20.0, 12.0);
    
    ctx.set_style(style);
}

fn configure_widget_styles(style: &mut egui::Style) {
    // Widgets estilo HUD tech
    style.visuals.widgets.noninteractive.bg_fill = HudColors::BG_PANEL;
    style.visuals.widgets.inactive.bg_fill = HudColors::BG_ELEVATED;
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(20, 50, 70);
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(0, 80, 100);
    
    // Bordas cyan tech
    style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, HudColors::CYAN_DARK);
    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.5, HudColors::CYAN_PRIMARY);
    style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(2.0, HudColors::CYAN_GLOW);
    style.visuals.widgets.active.bg_stroke = egui::Stroke::new(2.5, HudColors::CYAN_GLOW);
    
    // Cantos retos (tech/angular)
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(0.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(0.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(0.0);
    style.visuals.widgets.active.rounding = egui::Rounding::same(0.0);
}

/// Cria o ícone da aplicação com design HUD tech.
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
        
        // Círculos concêntricos estilo HUD
        if (dist > 10.0 && dist < 12.0) || (dist > 6.0 && dist < 7.0) {
            // Anéis cyan
            pixel[0] = 0;
            pixel[1] = 255;
            pixel[2] = 255;
            pixel[3] = 255;
        } else if dist < 4.0 {
            // Centro brilhante
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
