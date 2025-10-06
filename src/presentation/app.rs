use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::application::{ScanSystemUseCase, CleanSelectedCategoriesUseCase};
use crate::domain::entities::ScanResult;
use crate::infrastructure::{FileSystemScannerRepository, FileSystemCleanerRepository};
use crate::presentation::widgets::CategoryWidget;
use crate::presentation::theme::apply_cyberpunk_theme;
use crate::shared::format_bytes;

/// Aplicação principal - coordena a UI e os casos de uso.
/// Segue o padrão MVC/MVVM como Presenter/ViewModel.
pub struct PCCleanApp {
    // Use Cases (Application Layer)
    scan_use_case: Arc<ScanSystemUseCase>,
    clean_use_case: Arc<CleanSelectedCategoriesUseCase>,
    
    // Estado da aplicação
    scan_results: Arc<Mutex<Option<ScanResult>>>,
    selected_categories: Vec<bool>,
    
    // Estado de UI
    is_scanning: bool,
    is_cleaning: bool,
    show_confirmation: bool,
    animation_time: Instant,
}

impl PCCleanApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Injeção de dependências - criando as implementações concretas
        let scanner_repo = Arc::new(FileSystemScannerRepository::new());
        let cleaner_repo = Arc::new(FileSystemCleanerRepository::new());
        
        // Criando os use cases com as dependências injetadas
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

        let use_case = Arc::clone(&self.clean_use_case);
        let results = Arc::clone(&self.scan_results);
        
        // Coletar índices selecionados
        let selected_indices: Vec<usize> = self.selected_categories
            .iter()
            .enumerate()
            .filter_map(|(i, &selected)| if selected { Some(i) } else { None })
            .collect();

        std::thread::spawn(move || {
            if let Some(scan_result) = results.lock().unwrap().as_ref() {
                let _ = use_case.execute(scan_result, &selected_indices);
            }
        });
    }

    fn render_header(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                self.render_title(ui);
                self.render_status_indicator(ui);
            });
            ui.add_space(10.0);
        });
    }

    fn render_title(&self, ui: &mut egui::Ui) {
        let time = self.animation_time.elapsed().as_secs_f32();
        let glow = (time * 2.0).sin() * 0.3 + 0.7;
        
        ui.heading(
            egui::RichText::new("⚡ PCCLEAN")
                .size(36.0)
                .color(egui::Color32::from_rgb(
                    0,
                    (255.0 * glow) as u8,
                    (255.0 * glow) as u8,
                ))
        );
        
        ui.label(
            egui::RichText::new("Cyberpunk Disk Cleaner v2.0 - DDD Edition")
                .size(14.0)
                .color(egui::Color32::from_rgb(138, 43, 226))
        );
    }

    fn render_status_indicator(&self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(20.0);
            if self.is_scanning {
                ui.spinner();
                ui.label(
                    egui::RichText::new("SCANNING...")
                        .color(egui::Color32::from_rgb(0, 255, 255))
                );
            }
        });
    }

    fn render_status_bar(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                
                let status_color = if self.is_scanning || self.is_cleaning {
                    egui::Color32::from_rgb(255, 165, 0)
                } else {
                    egui::Color32::from_rgb(0, 255, 0)
                };
                
                ui.label(egui::RichText::new("●").size(12.0).color(status_color));
                
                self.render_total_size(ui);
                self.render_fps(ui);
            });
            ui.add_space(5.0);
        });
    }

    fn render_total_size(&self, ui: &mut egui::Ui) {
        if let Some(results) = self.scan_results.lock().unwrap().as_ref() {
            ui.label(
                egui::RichText::new(format!("Total encontrado: {}", format_bytes(results.total_size())))
                    .color(egui::Color32::from_rgb(138, 43, 226))
            );
        } else {
            ui.label(
                egui::RichText::new("Pronto para varredura")
                    .color(egui::Color32::GRAY)
            );
        }
    }

    fn render_fps(&self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(20.0);
            ui.label(
                egui::RichText::new(&format!("FPS: {:.0}", ui.input(|i| 1.0 / i.stable_dt)))
                    .color(egui::Color32::from_rgb(100, 100, 100))
            );
        });
    }

    fn render_action_buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add_space(20.0);
            self.render_scan_button(ui);
            ui.add_space(20.0);
            self.render_clean_button(ui);
        });
    }

    fn render_scan_button(&mut self, ui: &mut egui::Ui) {
        let scan_button = egui::Button::new(
            egui::RichText::new("🔍 INICIAR VARREDURA")
                .size(20.0)
                .color(egui::Color32::BLACK)
        )
        .fill(egui::Color32::from_rgb(0, 255, 255))
        .min_size(egui::vec2(250.0, 50.0));
        
        if ui.add_enabled(!self.is_scanning, scan_button).clicked() {
            self.start_scan();
        }
    }

    fn render_clean_button(&mut self, ui: &mut egui::Ui) {
        let has_selection = self.selected_categories.iter().any(|&x| x);
        let clean_button = egui::Button::new(
            egui::RichText::new("🧹 LIMPAR SELECIONADOS")
                .size(20.0)
                .color(egui::Color32::BLACK)
        )
        .fill(egui::Color32::from_rgb(255, 0, 255))
        .min_size(egui::vec2(250.0, 50.0));
        
        if ui.add_enabled(has_selection && !self.is_cleaning, clean_button).clicked() {
            self.show_confirmation = true;
        }
    }

    fn render_results(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            if let Some(results) = self.scan_results.lock().unwrap().as_ref() {
                self.is_scanning = false;
                let time = self.animation_time.elapsed().as_secs_f32();

                ui.add_space(10.0);
                for (index, category) in results.categories().iter().enumerate() {
                    let widget = CategoryWidget::new(
                        category,
                        index,
                        self.selected_categories.get(index).copied().unwrap_or(false),
                        time,
                    );
                    widget.render(ui, &mut self.selected_categories);
                }
            } else if !self.is_scanning {
                self.render_welcome_screen(ui);
            }
        });
    }

    fn render_welcome_screen(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.label(
                egui::RichText::new(
                    "╔═══════════════════════════════════════╗\n\
                     ║   ⚡ PCCLEAN DDD EDITION ⚡         ║\n\
                     ║                                       ║\n\
                     ║   Clean Architecture                  ║\n\
                     ║   Domain-Driven Design                ║\n\
                     ║   SOLID Principles                    ║\n\
                     ║                                       ║\n\
                     ║   Clique em INICIAR VARREDURA         ║\n\
                     ╚═══════════════════════════════════════╝"
                )
                .size(16.0)
                .color(egui::Color32::from_rgb(0, 255, 255))
                .family(egui::FontFamily::Monospace)
            );
        });
    }

    fn render_confirmation_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_confirmation {
            return;
        }

        egui::Window::new("⚠️ Confirmação")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new("Tem certeza que deseja limpar os itens selecionados?")
                        .size(16.0)
                        .color(egui::Color32::from_rgb(255, 165, 0))
                );
                ui.add_space(10.0);
                ui.label("Esta ação não pode ser desfeita!");
                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button(egui::RichText::new("✓ Confirmar").size(14.0)).clicked() {
                        self.start_cleaning();
                    }
                    if ui.button(egui::RichText::new("✗ Cancelar").size(14.0)).clicked() {
                        self.show_confirmation = false;
                    }
                });
            });
    }

    fn render_cleaning_progress(&mut self, ctx: &egui::Context) {
        if !self.is_cleaning {
            return;
        }

        egui::Window::new("🧹 Limpeza em Progresso")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.spinner();
                ui.label("Limpando arquivos...");
                ui.add_space(10.0);
            });

        // Simular conclusão (em produção, verificaria o estado real)
        if self.animation_time.elapsed().as_secs() > 3 {
            self.is_cleaning = false;
            self.scan_results = Arc::new(Mutex::new(None));
            self.selected_categories = vec![false; 5];
        }
    }
}

impl eframe::App for PCCleanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        apply_cyberpunk_theme(ctx);

        self.render_header(ctx);
        self.render_status_bar(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);
            self.render_action_buttons(ui);
            ui.add_space(20.0);
            self.render_results(ui);
        });

        self.render_confirmation_dialog(ctx);
        self.render_cleaning_progress(ctx);
    }
}
