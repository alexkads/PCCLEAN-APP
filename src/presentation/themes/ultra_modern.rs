use eframe::egui;
use egui::{Color32, FontFamily, FontId, Rounding, Shadow, Stroke, TextStyle};
use std::collections::BTreeMap;

/// Ultra Modern Theme - Design system inspirado em interfaces modernas
/// com glassmorphism, gradientes suaves e animações fluidas
pub struct UltraModernTheme;

impl UltraModernTheme {
    pub fn new() -> Self {
        Self
    }

    pub fn apply(&self, ctx: &egui::Context) {
        // Configurar fontes modernas
        self.configure_fonts(ctx);

        // Configurar estilo visual
        let mut style = (*ctx.style()).clone();
        self.configure_style(&mut style);
        ctx.set_style(style);
    }

    fn configure_fonts(&self, ctx: &egui::Context) {
        let fonts = egui::FontDefinitions::default();
        ctx.set_fonts(fonts);

        // Text styles modernos e elegantes
        let mut text_styles = BTreeMap::new();
        text_styles.insert(
            TextStyle::Small,
            FontId::new(11.0, FontFamily::Proportional),
        );
        text_styles.insert(
            TextStyle::Body,
            FontId::new(14.5, FontFamily::Proportional),
        );
        text_styles.insert(
            TextStyle::Button,
            FontId::new(15.0, FontFamily::Proportional),
        );
        text_styles.insert(
            TextStyle::Heading,
            FontId::new(24.0, FontFamily::Proportional),
        );
        text_styles.insert(
            TextStyle::Monospace,
            FontId::new(13.0, FontFamily::Monospace),
        );

        ctx.style_mut(move |style| {
            style.text_styles = text_styles.clone();
        });
    }

    fn configure_style(&self, style: &mut egui::Style) {
        // ========================================
        // CORES MODERNAS - Paleta Dark Elegant
        // ========================================

        // Background principal - Dark gradient
        let bg_primary = Color32::from_rgb(10, 10, 20);
        let bg_secondary = Color32::from_rgb(15, 15, 30);
        let bg_tertiary = Color32::from_rgb(20, 20, 40);

        // Accent colors - Purple/Blue gradient
        let accent_primary = Color32::from_rgb(139, 92, 246);   // Purple
        let accent_secondary = Color32::from_rgb(59, 130, 246); // Blue
        let accent_hover = Color32::from_rgb(167, 139, 250);    // Light purple

        // Status colors
        let _success = Color32::from_rgb(34, 197, 94);
        let _warning = Color32::from_rgb(251, 146, 60);
        let _danger = Color32::from_rgb(239, 68, 68);

        // Text colors
        let text_primary = Color32::from_rgb(243, 244, 246);
        let _text_secondary = Color32::from_rgb(156, 163, 175);
        let _text_muted = Color32::from_rgb(107, 114, 128);

        // ========================================
        // SPACING & LAYOUT
        // ========================================
        style.spacing.item_spacing = egui::vec2(12.0, 8.0);
        style.spacing.button_padding = egui::vec2(16.0, 10.0);
        style.spacing.window_margin = egui::Margin::same(16.0);
        style.spacing.menu_margin = egui::Margin::same(8.0);
        style.spacing.indent = 20.0;
        style.spacing.slider_width = 180.0;
        style.spacing.combo_width = 120.0;

        // ========================================
        // INTERAÇÃO - Animações suaves
        // ========================================
        style.interaction.selectable_labels = true;
        style.interaction.multi_widget_text_select = true;

        // ========================================
        // VISUALS - Glassmorphism & Shadows
        // ========================================
        let visuals = &mut style.visuals;

        // Dark mode elegante
        visuals.dark_mode = true;
        visuals.override_text_color = Some(text_primary);

        // Background com profundidade
        visuals.panel_fill = bg_primary;
        visuals.window_fill = Color32::from_rgba_premultiplied(15, 15, 30, 240); // Glassmorphism
        visuals.faint_bg_color = bg_secondary;
        visuals.extreme_bg_color = bg_tertiary;

        // Sombras suaves e elegantes (DROP SHADOWS)
        visuals.window_shadow = Shadow {
            offset: egui::vec2(0.0, 8.0),
            blur: 32.0,
            spread: 0.0,
            color: Color32::from_rgba_premultiplied(0, 0, 0, 100),
        };

        visuals.popup_shadow = Shadow {
            offset: egui::vec2(0.0, 4.0),
            blur: 16.0,
            spread: 0.0,
            color: Color32::from_rgba_premultiplied(0, 0, 0, 80),
        };

        // Borders suaves
        visuals.window_stroke = Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 10));
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 5));

        // ========================================
        // WIDGETS - Design moderno
        // ========================================

        // === BOTÕES ===
        // Estado normal
        visuals.widgets.inactive.weak_bg_fill = bg_secondary;
        visuals.widgets.inactive.bg_fill = accent_primary;
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.5, text_primary);
        visuals.widgets.inactive.rounding = Rounding::same(12.0);

        // Estado hover - efeito de elevação
        visuals.widgets.hovered.weak_bg_fill = bg_tertiary;
        visuals.widgets.hovered.bg_fill = accent_hover;
        visuals.widgets.hovered.fg_stroke = Stroke::new(2.0, text_primary);
        visuals.widgets.hovered.rounding = Rounding::same(12.0);
        visuals.widgets.hovered.expansion = 2.0; // Efeito de crescimento

        // Estado ativo/pressionado
        visuals.widgets.active.weak_bg_fill = accent_secondary;
        visuals.widgets.active.bg_fill = accent_secondary;
        visuals.widgets.active.fg_stroke = Stroke::new(2.0, text_primary);
        visuals.widgets.active.rounding = Rounding::same(12.0);

        // Estado open (menus, combos)
        visuals.widgets.open.weak_bg_fill = bg_tertiary;
        visuals.widgets.open.bg_fill = accent_secondary;
        visuals.widgets.open.fg_stroke = Stroke::new(2.0, accent_hover);
        visuals.widgets.open.rounding = Rounding::same(12.0);

        // === SELECTION ===
        visuals.selection.bg_fill = Color32::from_rgba_premultiplied(139, 92, 246, 60); // Purple translúcido
        visuals.selection.stroke = Stroke::new(1.5, accent_primary);

        // === HYPERLINKS ===
        visuals.hyperlink_color = accent_secondary;

        // === SCROLL BARS - Minimalistas ===
        visuals.widgets.inactive.bg_fill = Color32::from_rgba_premultiplied(255, 255, 255, 10);
        visuals.widgets.hovered.bg_fill = Color32::from_rgba_premultiplied(255, 255, 255, 20);
        visuals.widgets.active.bg_fill = accent_primary;

        // === CODE/MONOSPACE ===
        visuals.code_bg_color = Color32::from_rgb(25, 25, 45);

        // === EXTREMES ===
        visuals.extreme_bg_color = Color32::from_rgb(5, 5, 15);

        // === WINDOW ===
        visuals.window_rounding = Rounding::same(16.0);
        visuals.window_fill = Color32::from_rgba_premultiplied(15, 15, 30, 245);

        // === MENU ===
        visuals.menu_rounding = Rounding::same(12.0);

        // === RESIZE ===
        visuals.resize_corner_size = 12.0;

        // === CLIP RECT ===
        visuals.clip_rect_margin = 3.0;

        // === STRIPED ===
        visuals.striped = true;

        // === INDENT ===
        visuals.indent_has_left_vline = true;

        // === COLLAPSING HEADER ===
        visuals.collapsing_header_frame = true;
    }
}

impl Default for UltraModernTheme {
    fn default() -> Self {
        Self::new()
    }
}
