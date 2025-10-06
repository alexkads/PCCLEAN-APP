use eframe::egui;
use crate::domain::entities::CleanableCategory;
use crate::shared::format_bytes;

/// Renderiza uma categoria de itens limpáveis.
/// Componente reutilizável que segue o princípio de Single Responsibility.
pub struct CategoryWidget<'a> {
    category: &'a CleanableCategory,
    index: usize,
    is_selected: bool,
    animation_time: f32,
}

impl<'a> CategoryWidget<'a> {
    pub fn new(
        category: &'a CleanableCategory,
        index: usize,
        is_selected: bool,
        animation_time: f32,
    ) -> Self {
        Self {
            category,
            index,
            is_selected,
            animation_time,
        }
    }

    pub fn render(&self, ui: &mut egui::Ui, selected: &mut [bool]) {
        ui.add_space(10.0);
        
        // Borda HUD tech
        let border_color = if self.is_selected {
            egui::Color32::from_rgb(255, 50, 80) // Vermelho quando selecionado
        } else {
            egui::Color32::from_rgb(0, 180, 200) // Cyan padrão
        };

        // Frame angular estilo HUD
        egui::Frame::none()
            .fill(egui::Color32::from_rgba_premultiplied(10, 25, 35, 240))
            .stroke(egui::Stroke::new(2.0, border_color))
            .inner_margin(egui::Margin::same(12.0))
            .show(ui, |ui| {
                self.render_header(ui, selected);
                ui.add_space(10.0);
                
                // Linha separadora tech
                let width = ui.available_width();
                let rect_top = ui.cursor().min;
                ui.painter().line_segment(
                    [
                        egui::pos2(rect_top.x, rect_top.y),
                        egui::pos2(rect_top.x + width, rect_top.y)
                    ],
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 180, 200))
                );
                
                ui.add_space(10.0);
                self.render_items(ui);
            });
    }

    fn render_header(&self, ui: &mut egui::Ui, selected: &mut [bool]) {
        ui.horizontal(|ui| {
            // Checkbox HUD
            ui.checkbox(&mut selected[self.index], "");

            // Nome da categoria estilo HUD
            ui.label(
                egui::RichText::new(format!("[ {} ]", self.category.name()))
                    .size(14.0)
                    .color(egui::Color32::from_rgb(0, 255, 255))
                    .family(egui::FontFamily::Monospace)
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let size_color = self.get_size_color();
                
                // Tamanho estilo HUD
                ui.label(
                    egui::RichText::new(format_bytes(self.category.total_size()))
                        .size(16.0)
                        .color(size_color)
                        .family(egui::FontFamily::Monospace)
                        .strong()
                );

                ui.label(
                    egui::RichText::new("│")
                        .color(egui::Color32::from_rgb(0, 180, 200))
                );

                // Contagem de itens
                ui.label(
                    egui::RichText::new(format!("{} FILES", self.category.item_count()))
                        .size(11.0)
                        .color(egui::Color32::from_rgb(100, 200, 220))
                        .family(egui::FontFamily::Monospace)
                );
            });
        });
    }

    fn get_size_color(&self) -> egui::Color32 {
        let size = self.category.total_size();
        // Cores HUD para gravidade
        if size > 1_000_000_000 {
            egui::Color32::from_rgb(255, 50, 80) // Vermelho crítico > 1GB
        } else if size > 100_000_000 {
            egui::Color32::from_rgb(255, 150, 50) // Laranja warning > 100MB
        } else {
            egui::Color32::from_rgb(0, 255, 150) // Verde OK
        }
    }

    fn render_items(&self, ui: &mut egui::Ui) {
        const PREVIEW_COUNT: usize = 3;
        
        for (i, item) in self.category.take(PREVIEW_COUNT).iter().enumerate() {
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                
                // Indicador HUD
                ui.label(
                    egui::RichText::new("▸")
                        .size(12.0)
                        .color(egui::Color32::from_rgb(0, 255, 255))
                );
                
                ui.add_space(4.0);
                
                // Animação scan no primeiro item
                let alpha = if i == 0 {
                    ((self.animation_time * 3.0).sin() * 0.3 + 0.7) * 255.0
                } else {
                    180.0
                };
                
                // Path do arquivo estilo HUD
                ui.label(
                    egui::RichText::new(item.path())
                        .size(11.0)
                        .color(egui::Color32::from_rgba_premultiplied(100, 200, 220, alpha as u8))
                        .family(egui::FontFamily::Monospace)
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format_bytes(item.size_in_bytes()))
                            .size(10.0)
                            .color(egui::Color32::from_rgb(0, 255, 255))
                            .family(egui::FontFamily::Monospace)
                    );
                });
            });
            
            if i < PREVIEW_COUNT - 1 {
                ui.add_space(4.0);
            }
        }

        // Indicador de mais arquivos
        if self.category.item_count() > PREVIEW_COUNT {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new(format!("[+{} MORE FILES...]", self.category.item_count() - PREVIEW_COUNT))
                        .size(10.0)
                        .color(egui::Color32::from_rgb(100, 150, 170))
                        .family(egui::FontFamily::Monospace)
                );
            });
        }
    }
}
