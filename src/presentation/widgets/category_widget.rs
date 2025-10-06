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
        
        let frame_color = if self.is_selected {
            egui::Color32::from_rgb(255, 0, 255) // Magenta
        } else {
            egui::Color32::from_rgb(138, 43, 226) // Roxo
        };

        egui::Frame::none()
            .fill(egui::Color32::from_rgba_premultiplied(10, 10, 30, 200))
            .stroke(egui::Stroke::new(2.0, frame_color))
            .inner_margin(egui::Margin::same(15.0))
            .show(ui, |ui| {
                self.render_header(ui, selected);
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);
                self.render_items(ui);
            });
    }

    fn render_header(&self, ui: &mut egui::Ui, selected: &mut [bool]) {
        ui.horizontal(|ui| {
            ui.checkbox(&mut selected[self.index], "");

            ui.label(
                egui::RichText::new(self.category.name())
                    .size(18.0)
                    .color(egui::Color32::from_rgb(0, 255, 255))
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let size_color = self.get_size_color();
                
                ui.label(
                    egui::RichText::new(format_bytes(self.category.total_size()))
                        .size(16.0)
                        .color(size_color)
                );

                ui.label(
                    egui::RichText::new(format!("({} itens)", self.category.item_count()))
                        .color(egui::Color32::GRAY)
                );
            });
        });
    }

    fn get_size_color(&self) -> egui::Color32 {
        let size = self.category.total_size();
        if size > 1_000_000_000 {
            egui::Color32::from_rgb(255, 0, 0) // > 1GB
        } else if size > 100_000_000 {
            egui::Color32::from_rgb(255, 165, 0) // > 100MB
        } else {
            egui::Color32::from_rgb(0, 255, 0) // Verde
        }
    }

    fn render_items(&self, ui: &mut egui::Ui) {
        const PREVIEW_COUNT: usize = 3;
        
        for (i, item) in self.category.take(PREVIEW_COUNT).iter().enumerate() {
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                
                let alpha = if i == 0 {
                    ((self.animation_time * 3.0).sin() * 0.5 + 0.5) * 255.0
                } else {
                    200.0
                };
                
                ui.label(
                    egui::RichText::new(item.path())
                        .color(egui::Color32::from_rgba_premultiplied(150, 150, 150, alpha as u8))
                        .family(egui::FontFamily::Monospace)
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format_bytes(item.size_in_bytes()))
                            .color(egui::Color32::GRAY)
                    );
                });
            });
        }

        if self.category.item_count() > PREVIEW_COUNT {
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.label(
                    egui::RichText::new(format!("... e mais {} itens", self.category.item_count() - PREVIEW_COUNT))
                        .color(egui::Color32::DARK_GRAY)
                        .italics()
                );
            });
        }
    }
}
