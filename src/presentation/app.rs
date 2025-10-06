use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::application::{ScanSystemUseCase, CleanSelectedCategoriesUseCase};
use crate::domain::entities::ScanResult;
use crate::infrastructure::{FileSystemScannerRepository, FileSystemCleanerRepository};
use crate::presentation::themes::ThemeManager;
use crate::shared::format_bytes;

/// Mensagens de estado da limpeza
enum CleaningMessage {
    InProgress,
    Completed(usize),
    Failed(String),
}

/// Aplicação principal com design ultra moderno
pub struct PCCleanApp {
    // Use Cases
    scan_use_case: Arc<ScanSystemUseCase>,
    clean_use_case: Arc<CleanSelectedCategoriesUseCase>,

    // Estado
    scan_results: Arc<Mutex<Option<ScanResult>>>,
    selected_categories: Vec<bool>,

    // UI State
    is_scanning: bool,
    is_cleaning: bool,
    show_confirmation: bool,
    animation_time: Instant,

    // Cleaning channel
    cleaning_receiver: Option<Receiver<CleaningMessage>>,
    cleaning_status: String,

    // Theme
    theme_manager: ThemeManager,
}

impl PCCleanApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configurar visual hints para melhor qualidade
        cc.egui_ctx.set_visuals(egui::Visuals::dark());

        // Injeção de dependências
        let scanner_repo = Arc::new(FileSystemScannerRepository::new());
        let cleaner_repo = Arc::new(FileSystemCleanerRepository::new());

        let scan_use_case = Arc::new(ScanSystemUseCase::new(scanner_repo));
        let clean_use_case = Arc::new(CleanSelectedCategoriesUseCase::new(cleaner_repo));

        Self {
            scan_use_case,
            clean_use_case,
            scan_results: Arc::new(Mutex::new(None)),
            selected_categories: vec![false; 5],
            is_scanning: false,
            is_cleaning: false,
            show_confirmation: false,
            animation_time: Instant::now(),
            cleaning_receiver: None,
            cleaning_status: String::new(),
            theme_manager: ThemeManager::new(),
        }
    }

    fn start_scan(&mut self) {
        if self.is_scanning {
            return;
        }

        self.is_scanning = true;

        let use_case = Arc::clone(&self.scan_use_case);
        let results = Arc::clone(&self.scan_results);

        std::thread::spawn(move || {
            if let Ok(result) = use_case.execute() {
                *results.lock().unwrap() = Some(result);
            }
        });
    }

    fn start_cleaning(&mut self) {
        if self.is_cleaning || !self.show_confirmation {
            return;
        }

        self.is_cleaning = true;
        self.show_confirmation = false;
        self.cleaning_status = "Iniciando limpeza...".to_string();

        let use_case = Arc::clone(&self.clean_use_case);
        let results = Arc::clone(&self.scan_results);

        let selected_indices: Vec<usize> = self.selected_categories
            .iter()
            .enumerate()
            .filter_map(|(i, &selected)| if selected { Some(i) } else { None })
            .collect();

        let (sender, receiver): (Sender<CleaningMessage>, Receiver<CleaningMessage>) = channel();
        self.cleaning_receiver = Some(receiver);

        std::thread::spawn(move || {
            sender.send(CleaningMessage::InProgress).ok();

            if let Some(scan_result) = results.lock().unwrap().as_ref() {
                match use_case.execute(scan_result, &selected_indices) {
                    Ok(cleaned_count) => {
                        sender.send(CleaningMessage::Completed(cleaned_count)).ok();
                    }
                    Err(e) => {
                        sender.send(CleaningMessage::Failed(e.to_string())).ok();
                    }
                }
            } else {
                sender.send(CleaningMessage::Failed("Nenhum resultado de scan disponível".to_string())).ok();
            }
        });
    }

    /// Renderiza header moderno com gradiente
    fn render_header(&self, ui: &mut egui::Ui) {
        let time = self.animation_time.elapsed().as_secs_f32();

        ui.add_space(20.0);

        // Frame glassmorphism
        egui::Frame::none()
            .fill(egui::Color32::from_rgba_premultiplied(20, 20, 40, 200))
            .rounding(egui::Rounding::same(16.0))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(139, 92, 246, 60)))
            .inner_margin(egui::Margin::symmetric(40.0, 30.0))
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    // Logo animado
                    let pulse = (time * 2.0).sin() * 0.15 + 0.85;

                    ui.heading(
                        egui::RichText::new("✨ PCCLEAN")
                            .size(48.0)
                            .color(egui::Color32::from_rgb(
                                (139.0 * pulse) as u8,
                                (92.0 * pulse) as u8,
                                (246.0 * pulse) as u8,
                            ))
                    );

                    ui.add_space(8.0);

                    // Subtitle com gradiente simulado
                    ui.label(
                        egui::RichText::new("Ultra Modern Disk Cleaner")
                            .size(16.0)
                            .color(egui::Color32::from_rgb(156, 163, 175))
                    );

                    ui.add_space(4.0);

                    // Tag line
                    ui.label(
                        egui::RichText::new("DDD Architecture • Clean Code • SOLID")
                            .size(11.0)
                            .color(egui::Color32::from_rgb(107, 114, 128))
                    );
                });
            });

        ui.add_space(20.0);
    }

    /// Renderiza status bar moderno
    fn render_status_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar")
            .frame(egui::Frame::none()
                .fill(egui::Color32::from_rgba_premultiplied(15, 15, 30, 240))
                .inner_margin(egui::Margin::symmetric(20.0, 12.0)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Status indicator
                    let (status_color, status_text) = if self.is_scanning {
                        (egui::Color32::from_rgb(251, 146, 60), "SCANNING")
                    } else if self.is_cleaning {
                        (egui::Color32::from_rgb(251, 146, 60), "CLEANING")
                    } else {
                        (egui::Color32::from_rgb(34, 197, 94), "READY")
                    };

                    let time = self.animation_time.elapsed().as_secs_f32();
                    let pulse = if self.is_scanning || self.is_cleaning {
                        (time * 3.0).sin() * 0.3 + 0.7
                    } else {
                        1.0
                    };

                    ui.label(
                        egui::RichText::new("●")
                            .size(14.0)
                            .color(egui::Color32::from_rgba_premultiplied(
                                status_color.r(),
                                status_color.g(),
                                status_color.b(),
                                (255.0 * pulse) as u8
                            ))
                    );

                    ui.label(
                        egui::RichText::new(status_text)
                            .size(12.0)
                            .color(egui::Color32::from_rgb(156, 163, 175))
                    );

                    ui.separator();

                    // Total size
                    if let Some(results) = self.scan_results.lock().unwrap().as_ref() {
                        ui.label(
                            egui::RichText::new(format!("📦 {}", format_bytes(results.total_size())))
                                .size(12.0)
                                .color(egui::Color32::from_rgb(139, 92, 246))
                        );

                        ui.separator();

                        ui.label(
                            egui::RichText::new(format!("📁 {} items", results.total_items()))
                                .size(12.0)
                                .color(egui::Color32::from_rgb(59, 130, 246))
                        );
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // FPS counter
                        ui.label(
                            egui::RichText::new(format!("{:.0} FPS", ui.input(|i| 1.0 / i.stable_dt)))
                                .size(11.0)
                                .color(egui::Color32::from_rgb(107, 114, 128))
                        );
                    });
                });
            });
    }

    /// Renderiza card de stats com visual moderno
    fn render_stats_card(&self, ui: &mut egui::Ui) {
        let time = self.animation_time.elapsed().as_secs_f32();

        egui::Frame::none()
            .fill(egui::Color32::from_rgba_premultiplied(20, 20, 40, 180))
            .rounding(egui::Rounding::same(20.0))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(139, 92, 246, 40)))
            .inner_margin(egui::Margin::symmetric(32.0, 24.0))
            .show(ui, |ui| {
                if let Some(results) = self.scan_results.lock().unwrap().as_ref() {
                    ui.vertical_centered(|ui| {
                        // Total size - número grande e sexy
                        let total_gb = results.total_size() as f64 / (1024.0 * 1024.0 * 1024.0);

                        ui.label(
                            egui::RichText::new(format!("{:.2}", total_gb))
                                .size(64.0)
                                .color(egui::Color32::from_rgb(139, 92, 246))
                        );

                        ui.label(
                            egui::RichText::new("GB")
                                .size(20.0)
                                .color(egui::Color32::from_rgb(107, 114, 128))
                        );

                        ui.add_space(12.0);

                        // Items count
                        ui.label(
                            egui::RichText::new(format!("{} files detected", results.total_items()))
                                .size(14.0)
                                .color(egui::Color32::from_rgb(156, 163, 175))
                        );
                    });
                } else {
                    ui.vertical_centered(|ui| {
                        // Círculo de scan animado
                        let (response, painter) = ui.allocate_painter(
                            egui::vec2(100.0, 100.0),
                            egui::Sense::hover()
                        );

                        let center = response.rect.center();

                        // Círculos concêntricos pulsantes
                        for i in 0..4 {
                            let radius = 40.0 - (i as f32 * 8.0);
                            let phase = time * 2.0 + i as f32 * 0.5;
                            let alpha = ((phase.sin() + 1.0) * 100.0 + 55.0) as u8;

                            painter.circle_stroke(
                                center,
                                radius,
                                egui::Stroke::new(2.0, egui::Color32::from_rgba_premultiplied(139, 92, 246, alpha))
                            );
                        }

                        ui.add_space(16.0);

                        ui.label(
                            egui::RichText::new("Ready to scan")
                                .size(16.0)
                                .color(egui::Color32::from_rgb(156, 163, 175))
                        );
                    });
                }
            });
    }

    /// Botões modernos com hover effects
    fn render_action_buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let button_size = egui::vec2(200.0, 56.0);

            // Scan button
            let scan_btn = egui::Button::new(
                egui::RichText::new(if self.is_scanning { "⏳ Scanning..." } else { "🔍 Start Scan" })
                    .size(16.0)
            )
            .fill(egui::Color32::from_rgb(139, 92, 246))
            .min_size(button_size)
            .rounding(egui::Rounding::same(12.0));

            if ui.add_enabled(!self.is_scanning, scan_btn).clicked() {
                self.start_scan();
            }

            ui.add_space(16.0);

            // Clean button
            let has_selection = self.selected_categories.iter().any(|&x| x);
            let clean_btn = egui::Button::new(
                egui::RichText::new("🗑️ Clean Selected")
                    .size(16.0)
            )
            .fill(egui::Color32::from_rgb(239, 68, 68))
            .min_size(button_size)
            .rounding(egui::Rounding::same(12.0));

            if ui.add_enabled(has_selection && !self.is_cleaning, clean_btn).clicked() {
                self.show_confirmation = true;
            }
        });
    }

    /// Renderiza lista de categorias com cards modernos
    fn render_category_cards(&mut self, ui: &mut egui::Ui) {
        let has_results = self.scan_results.lock().unwrap().is_some();

        if has_results {
            self.is_scanning = false;

            // Clone categories para evitar problemas de borrow
            let categories = self.scan_results.lock().unwrap()
                .as_ref()
                .map(|r| r.categories().to_vec())
                .unwrap_or_default();

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for (index, category) in categories.iter().enumerate() {
                        self.render_category_card(ui, category, index);
                        ui.add_space(12.0);
                    }
                });
        } else if !self.is_scanning {
            self.render_welcome_message(ui);
        } else {
            // Loading state
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                ui.spinner();
                ui.add_space(16.0);
                ui.label(
                    egui::RichText::new("Scanning your system...")
                        .size(16.0)
                        .color(egui::Color32::from_rgb(156, 163, 175))
                );
            });
        }
    }

    /// Card individual de categoria - design sexy
    fn render_category_card(&mut self, ui: &mut egui::Ui, category: &crate::domain::entities::CleanableCategory, index: usize) {
        let is_selected = self.selected_categories.get(index).copied().unwrap_or(false);

        let card_bg = if is_selected {
            egui::Color32::from_rgba_premultiplied(139, 92, 246, 40)
        } else {
            egui::Color32::from_rgba_premultiplied(20, 20, 40, 150)
        };

        let border_color = if is_selected {
            egui::Color32::from_rgb(139, 92, 246)
        } else {
            egui::Color32::from_rgba_premultiplied(255, 255, 255, 10)
        };

        egui::Frame::none()
            .fill(card_bg)
            .rounding(egui::Rounding::same(16.0))
            .stroke(egui::Stroke::new(1.5, border_color))
            .inner_margin(egui::Margin::symmetric(20.0, 16.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Checkbox customizado
                    let mut selected = is_selected;
                    if ui.checkbox(&mut selected, "").changed() {
                        if index < self.selected_categories.len() {
                            self.selected_categories[index] = selected;
                        }
                    }

                    ui.add_space(8.0);

                    // Ícone da categoria
                    let icon = match category.name().to_lowercase().as_str() {
                        name if name.contains("log") => "📋",
                        name if name.contains("cache") => "💾",
                        name if name.contains("download") => "⬇️",
                        name if name.contains("temp") => "🗂️",
                        name if name.contains("docker") => "🐳",
                        _ => "📁",
                    };

                    ui.label(egui::RichText::new(icon).size(24.0));

                    ui.add_space(12.0);

                    // Info
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(category.name())
                                .size(16.0)
                                .color(egui::Color32::from_rgb(243, 244, 246))
                        );

                        ui.label(
                            egui::RichText::new(format!("{} items • {}",
                                category.items().len(),
                                format_bytes(category.total_size())
                            ))
                            .size(13.0)
                            .color(egui::Color32::from_rgb(156, 163, 175))
                        );
                    });

                    // Size badge
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let size_gb = category.total_size() as f64 / (1024.0 * 1024.0 * 1024.0);

                        egui::Frame::none()
                            .fill(egui::Color32::from_rgb(139, 92, 246))
                            .rounding(egui::Rounding::same(8.0))
                            .inner_margin(egui::Margin::symmetric(12.0, 6.0))
                            .show(ui, |ui| {
                                ui.label(
                                    egui::RichText::new(format!("{:.2} GB", size_gb))
                                        .size(14.0)
                                        .color(egui::Color32::WHITE)
                                );
                            });
                    });
                });
            });
    }

    /// Welcome screen minimalista
    fn render_welcome_message(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(80.0);

            ui.label(
                egui::RichText::new("👋")
                    .size(64.0)
            );

            ui.add_space(16.0);

            ui.label(
                egui::RichText::new("Welcome to PCCLEAN")
                    .size(24.0)
                    .color(egui::Color32::from_rgb(243, 244, 246))
            );

            ui.add_space(8.0);

            ui.label(
                egui::RichText::new("Click 'Start Scan' to analyze your system")
                    .size(14.0)
                    .color(egui::Color32::from_rgb(156, 163, 175))
            );
        });
    }

    /// Modal de confirmação moderno
    fn render_confirmation_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_confirmation {
            return;
        }

        egui::Window::new("Confirm Action")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(egui::Frame::none()
                .fill(egui::Color32::from_rgba_premultiplied(20, 20, 40, 250))
                .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgb(139, 92, 246)))
                .rounding(egui::Rounding::same(20.0))
                .inner_margin(egui::Margin::same(32.0)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(egui::RichText::new("⚠️").size(48.0));

                    ui.add_space(16.0);

                    ui.label(
                        egui::RichText::new("Confirm Cleanup")
                            .size(24.0)
                            .color(egui::Color32::from_rgb(243, 244, 246))
                    );

                    ui.add_space(12.0);

                    ui.label(
                        egui::RichText::new("Are you sure you want to delete the selected files?")
                            .size(14.0)
                            .color(egui::Color32::from_rgb(156, 163, 175))
                    );

                    ui.add_space(8.0);

                    ui.label(
                        egui::RichText::new("This action cannot be undone")
                            .size(13.0)
                            .color(egui::Color32::from_rgb(239, 68, 68))
                    );

                    ui.add_space(24.0);

                    ui.horizontal(|ui| {
                        let btn_size = egui::vec2(140.0, 44.0);

                        let confirm_btn = egui::Button::new(
                            egui::RichText::new("✓ Confirm").size(15.0)
                        )
                        .fill(egui::Color32::from_rgb(239, 68, 68))
                        .min_size(btn_size)
                        .rounding(egui::Rounding::same(12.0));

                        if ui.add(confirm_btn).clicked() {
                            self.start_cleaning();
                        }

                        ui.add_space(12.0);

                        let cancel_btn = egui::Button::new(
                            egui::RichText::new("✗ Cancel").size(15.0)
                        )
                        .fill(egui::Color32::from_rgb(75, 85, 99))
                        .min_size(btn_size)
                        .rounding(egui::Rounding::same(12.0));

                        if ui.add(cancel_btn).clicked() {
                            self.show_confirmation = false;
                        }
                    });
                });
            });
    }

    /// Progress dialog para limpeza
    fn render_cleaning_progress(&mut self, ctx: &egui::Context) {
        if let Some(receiver) = &self.cleaning_receiver {
            if let Ok(msg) = receiver.try_recv() {
                match msg {
                    CleaningMessage::InProgress => {
                        self.cleaning_status = "Removing files...".to_string();
                    }
                    CleaningMessage::Completed(count) => {
                        self.cleaning_status = format!("✓ Completed! {} items removed", count);
                        self.is_cleaning = false;
                        self.cleaning_receiver = None;
                        self.scan_results = Arc::new(Mutex::new(None));
                        self.selected_categories = vec![false; 5];
                    }
                    CleaningMessage::Failed(err) => {
                        self.cleaning_status = format!("✗ Error: {}", err);
                        self.is_cleaning = false;
                        self.cleaning_receiver = None;
                    }
                }
            }
        }

        if !self.is_cleaning && !self.cleaning_status.starts_with('✓') && !self.cleaning_status.starts_with('✗') {
            return;
        }

        egui::Window::new("Cleaning")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(egui::Frame::none()
                .fill(egui::Color32::from_rgba_premultiplied(20, 20, 40, 250))
                .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgb(139, 92, 246)))
                .rounding(egui::Rounding::same(20.0))
                .inner_margin(egui::Margin::same(40.0)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    if self.is_cleaning {
                        ui.spinner();
                        ui.add_space(20.0);
                        ui.label(
                            egui::RichText::new("Processing...")
                                .size(20.0)
                                .color(egui::Color32::from_rgb(243, 244, 246))
                        );
                    } else {
                        ui.label(egui::RichText::new("✓").size(48.0).color(egui::Color32::from_rgb(34, 197, 94)));
                        ui.add_space(16.0);
                        ui.label(
                            egui::RichText::new("Cleanup Complete")
                                .size(20.0)
                                .color(egui::Color32::from_rgb(34, 197, 94))
                        );
                    }

                    ui.add_space(16.0);

                    ui.label(
                        egui::RichText::new(&self.cleaning_status)
                            .size(14.0)
                            .color(egui::Color32::from_rgb(156, 163, 175))
                    );

                    if !self.is_cleaning {
                        ui.add_space(24.0);
                        let close_btn = egui::Button::new(
                            egui::RichText::new("Close").size(15.0)
                        )
                        .fill(egui::Color32::from_rgb(139, 92, 246))
                        .min_size(egui::vec2(120.0, 40.0))
                        .rounding(egui::Rounding::same(12.0));

                        if ui.add(close_btn).clicked() {
                            self.cleaning_status = String::new();
                        }
                    }
                });
            });
    }
}

impl eframe::App for PCCleanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        // Aplicar tema ultra moderno
        self.theme_manager.apply(ctx);

        // Status bar
        self.render_status_bar(ctx);

        // Main panel
        egui::CentralPanel::default()
            .frame(egui::Frame::none()
                .fill(egui::Color32::from_rgb(10, 10, 20))
                .inner_margin(egui::Margin::symmetric(40.0, 30.0)))
            .show(ctx, |ui| {
                // Header
                self.render_header(ui);

                ui.add_space(24.0);

                // Stats card
                self.render_stats_card(ui);

                ui.add_space(24.0);

                // Action buttons
                ui.vertical_centered(|ui| {
                    self.render_action_buttons(ui);
                });

                ui.add_space(32.0);

                // Categories
                self.render_category_cards(ui);
            });

        // Modals
        self.render_confirmation_dialog(ctx);
        self.render_cleaning_progress(ctx);
    }
}
