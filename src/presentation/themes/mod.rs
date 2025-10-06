mod ultra_modern;

pub use ultra_modern::UltraModernTheme;

use eframe::egui;

/// Sistema de tema único e impressionante
pub struct ThemeManager {
    theme: UltraModernTheme,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            theme: UltraModernTheme::new(),
        }
    }

    pub fn apply(&self, ctx: &egui::Context) {
        self.theme.apply(ctx);
    }
}
