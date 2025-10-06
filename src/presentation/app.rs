use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::application::{ScanSystemUseCase, CleanSelectedCategoriesUseCase};
use crate::domain::entities::ScanResult;
use crate::infrastructure::{FileSystemScannerRepository, FileSystemCleanerRepository};
use crate::presentation::widgets::CategoryWidget;
use crate::presentation::theme::apply_cyberpunk_theme;
use crate::shared::format_bytes;

/// Mensagens de estado da limpeza
enum CleaningMessage {
    InProgress,
    Completed(usize), // número de itens limpos
    Failed(String),
}

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
    
    // Canal de comunicação com thread de limpeza
    cleaning_receiver: Option<Receiver<CleaningMessage>>,
    cleaning_status: String,
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
            cleaning_receiver: None,
            cleaning_status: String::new(),
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
        
        // Coletar índices selecionados
        let selected_indices: Vec<usize> = self.selected_categories
            .iter()
            .enumerate()
            .filter_map(|(i, &selected)| if selected { Some(i) } else { None })
            .collect();

        // Criar canal de comunicação
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

    fn render_header(&self, ui: &mut egui::Ui) {
        let time = self.animation_time.elapsed().as_secs_f32();
        
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            
            // Título HUD tech com animação
            let pulse = (time * 2.0).sin() * 0.2 + 0.8;
            ui.heading(
                egui::RichText::new("◢ SYSTEM SCANNER ◣")
                    .size(36.0)
                    .color(egui::Color32::from_rgba_premultiplied(
                        0, 255, 255,
                        (255.0 * pulse) as u8
                    ))
                    .family(egui::FontFamily::Monospace)
            );
            
            ui.add_space(5.0);
            
            // Status bar tech
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("▌").color(egui::Color32::from_rgb(0, 255, 255)));
                ui.label(
                    egui::RichText::new("DISK OPTIMIZATION MODULE v2.077")
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 200, 220))
                        .family(egui::FontFamily::Monospace)
                );
                ui.label(egui::RichText::new("▐").color(egui::Color32::from_rgb(0, 255, 255)));
            });
            
            ui.add_space(10.0);
            
            // Linha decorativa tech
            let width = ui.available_width();
            let rect_top = ui.cursor().min;
            ui.painter().line_segment(
                [
                    egui::pos2(rect_top.x, rect_top.y),
                    egui::pos2(rect_top.x + width, rect_top.y)
                ],
                egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 255, 255))
            );
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
        let time = self.animation_time.elapsed().as_secs_f32();
        let pulse = ((time * 3.0).sin() + 1.0) / 2.0;
        
        let scan_button = egui::Button::new(
            egui::RichText::new("▶ INITIALIZE SCAN")
                .size(16.0)
                .family(egui::FontFamily::Monospace)
        )
        .fill(egui::Color32::from_rgb(5, 30, 40))
        .stroke(egui::Stroke::new(
            2.0,
            egui::Color32::from_rgba_premultiplied(0, 255, 255, (200.0 * pulse + 55.0) as u8)
        ))
        .min_size(egui::vec2(220.0, 50.0));
        
        if ui.add_enabled(!self.is_scanning, scan_button).clicked() {
            self.start_scan();
        }
    }

    fn render_clean_button(&mut self, ui: &mut egui::Ui) {
        let has_selection = self.selected_categories.iter().any(|&x| x);
        let clean_button = egui::Button::new(
            egui::RichText::new("▼ EXECUTE CLEANUP")
                .size(16.0)
                .family(egui::FontFamily::Monospace)
        )
        .fill(egui::Color32::from_rgb(40, 10, 10))
        .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 50, 80)))
        .min_size(egui::vec2(220.0, 50.0));
        
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
        let time = self.animation_time.elapsed().as_secs_f32();
        
        ui.vertical_centered(|ui| {
            ui.add_space(60.0);
            
            // Frame HUD estilo tech
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(10, 25, 35))
                .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(0, 255, 255)))
                .inner_margin(egui::Margin::same(40.0))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        // Círculo de scan animado
                        let (response, painter) = ui.allocate_painter(
                            egui::vec2(120.0, 120.0),
                            egui::Sense::hover()
                        );
                        
                        let center = response.rect.center();
                        
                        // Círculos concêntricos animados
                        for i in 0..5 {
                            let radius = 50.0 - (i as f32 * 8.0);
                            let alpha = ((time * 3.0 + i as f32).sin() * 80.0 + 175.0) as u8;
                            painter.circle_stroke(
                                center,
                                radius,
                                egui::Stroke::new(2.0, egui::Color32::from_rgba_premultiplied(0, 255, 255, alpha))
                            );
                        }
                        
                        // Linha de scan rotativa
                        let scan_angle = time * 2.0;
                        let x = center.x + scan_angle.cos() * 50.0;
                        let y = center.y + scan_angle.sin() * 50.0;
                        painter.line_segment(
                            [center, egui::pos2(x, y)],
                            egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 50, 80))
                        );
                        
                        ui.add_space(20.0);
                        
                        ui.label(
                            egui::RichText::new("╔═══════════════════════════════════╗")
                                .color(egui::Color32::from_rgb(0, 255, 255))
                                .family(egui::FontFamily::Monospace)
                        );
                        
                        ui.label(
                            egui::RichText::new("║  SYSTEM OPTIMIZATION PROTOCOL   ║")
                                .size(14.0)
                                .color(egui::Color32::from_rgb(200, 240, 255))
                                .family(egui::FontFamily::Monospace)
                        );
                        
                        ui.label(
                            egui::RichText::new("╠═══════════════════════════════════╣")
                                .color(egui::Color32::from_rgb(0, 255, 255))
                                .family(egui::FontFamily::Monospace)
                        );
                        
                        ui.add_space(10.0);
                        
                        ui.label(
                            egui::RichText::new("║  ▸ Domain-Driven Architecture   ║")
                                .size(11.0)
                                .color(egui::Color32::from_rgb(100, 200, 220))
                                .family(egui::FontFamily::Monospace)
                        );
                        
                        ui.label(
                            egui::RichText::new("║  ▸ Clean Code Principles        ║")
                                .size(11.0)
                                .color(egui::Color32::from_rgb(100, 200, 220))
                                .family(egui::FontFamily::Monospace)
                        );
                        
                        ui.label(
                            egui::RichText::new("║  ▸ SOLID Design Pattern         ║")
                                .size(11.0)
                                .color(egui::Color32::from_rgb(100, 200, 220))
                                .family(egui::FontFamily::Monospace)
                        );
                        
                        ui.add_space(10.0);
                        
                        ui.label(
                            egui::RichText::new("╚═══════════════════════════════════╝")
                                .color(egui::Color32::from_rgb(0, 255, 255))
                                .family(egui::FontFamily::Monospace)
                        );
                        
                        ui.add_space(20.0);
                        
                        // Texto piscante
                        let blink = ((time * 2.0).sin() + 1.0) / 2.0;
                        ui.label(
                            egui::RichText::new(">>> PRESS INITIALIZE SCAN TO BEGIN")
                                .size(11.0)
                                .color(egui::Color32::from_rgba_premultiplied(
                                    0, 255, 255,
                                    (255.0 * blink) as u8
                                ))
                                .family(egui::FontFamily::Monospace)
                        );
                    });
                });
        });
    }

    fn render_confirmation_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_confirmation {
            return;
        }

        egui::Window::new("⚠️  Confirmação")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(egui::Frame::none()
                .fill(egui::Color32::from_rgb(31, 41, 55))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(75, 85, 99)))
                .rounding(egui::Rounding::same(12.0))
                .inner_margin(egui::Margin::same(24.0)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Confirmar Limpeza")
                            .size(20.0)
                            .color(egui::Color32::from_rgb(243, 244, 246))
                            .strong()
                    );
                    
                    ui.add_space(16.0);
                    
                    ui.label(
                        egui::RichText::new("Deseja realmente excluir os arquivos selecionados?")
                            .size(14.0)
                            .color(egui::Color32::from_rgb(156, 163, 175))
                    );
                    
                    ui.add_space(8.0);
                    
                    ui.label(
                        egui::RichText::new("Esta ação não pode ser desfeita.")
                            .size(13.0)
                            .color(egui::Color32::from_rgb(239, 68, 68))
                    );
                    
                    ui.add_space(24.0);

                    ui.horizontal(|ui| {
                        let confirm_btn = egui::Button::new(
                            egui::RichText::new("✓  Confirmar")
                                .size(14.0)
                        )
                        .fill(egui::Color32::from_rgb(34, 197, 94))
                        .min_size(egui::vec2(120.0, 40.0));
                        
                        if ui.add(confirm_btn).clicked() {
                            self.start_cleaning();
                        }
                        
                        ui.add_space(12.0);
                        
                        let cancel_btn = egui::Button::new(
                            egui::RichText::new("✗  Cancelar")
                                .size(14.0)
                        )
                        .fill(egui::Color32::from_rgb(75, 85, 99))
                        .min_size(egui::vec2(120.0, 40.0));
                        
                        if ui.add(cancel_btn).clicked() {
                            self.show_confirmation = false;
                        }
                    });
                });
            });
    }

    fn render_cleaning_progress(&mut self, ctx: &egui::Context) {
        // Verificar mensagens da thread de limpeza
        if let Some(receiver) = &self.cleaning_receiver {
            if let Ok(msg) = receiver.try_recv() {
                match msg {
                    CleaningMessage::InProgress => {
                        self.cleaning_status = "Excluindo arquivos...".to_string();
                    }
                    CleaningMessage::Completed(count) => {
                        self.cleaning_status = format!("✓ Concluído! {} itens removidos", count);
                        self.is_cleaning = false;
                        self.cleaning_receiver = None;
                        
                        // Limpar resultados e seleções
                        self.scan_results = Arc::new(Mutex::new(None));
                        self.selected_categories = vec![false; 5];
                    }
                    CleaningMessage::Failed(err) => {
                        self.cleaning_status = format!("✗ Erro: {}", err);
                        self.is_cleaning = false;
                        self.cleaning_receiver = None;
                    }
                }
            }
        }
        
        if !self.is_cleaning && !self.cleaning_status.starts_with('✓') && !self.cleaning_status.starts_with('✗') {
            return;
        }

        egui::Window::new("🔄  Limpando")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(egui::Frame::none()
                .fill(egui::Color32::from_rgb(31, 41, 55))
                .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(75, 85, 99)))
                .rounding(egui::Rounding::same(12.0))
                .inner_margin(egui::Margin::same(32.0)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    
                    if self.is_cleaning {
                        ui.label(
                            egui::RichText::new("Processando...")
                                .size(18.0)
                                .color(egui::Color32::from_rgb(243, 244, 246))
                        );
                        
                        ui.add_space(20.0);
                        ui.spinner();
                        ui.add_space(20.0);
                    } else {
                        ui.label(
                            egui::RichText::new("Limpeza Concluída")
                                .size(18.0)
                                .color(egui::Color32::from_rgb(34, 197, 94))
                        );
                        
                        ui.add_space(20.0);
                    }
                    
                    ui.label(
                        egui::RichText::new(&self.cleaning_status)
                            .size(14.0)
                            .color(egui::Color32::from_rgb(156, 163, 175))
                    );
                    
                    if !self.is_cleaning {
                        ui.add_space(20.0);
                        let close_btn = egui::Button::new(
                            egui::RichText::new("Fechar")
                                .size(14.0)
                        )
                        .fill(egui::Color32::from_rgb(99, 102, 241))
                        .min_size(egui::vec2(100.0, 36.0));
                        
                        if ui.add(close_btn).clicked() {
                            self.cleaning_status = String::new();
                        }
                    }
                    
                    ui.add_space(10.0);
                });
            });
    }
    
    // Renderiza grid de fundo estilo HUD tech
    fn render_tech_grid(&self, ctx: &egui::Context) {
        let painter = ctx.layer_painter(egui::LayerId::background());
        let rect = ctx.screen_rect();
        let time = self.animation_time.elapsed().as_secs_f32();
        
        // Grid horizontal
        for i in (0..rect.height() as i32).step_by(30) {
            let y = rect.top() + i as f32;
            let alpha = 15 + ((time + i as f32 * 0.1).sin() * 5.0) as u8;
            painter.line_segment(
                [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                egui::Stroke::new(0.5, egui::Color32::from_rgba_premultiplied(0, 255, 255, alpha))
            );
        }
        
        // Grid vertical
        for i in (0..rect.width() as i32).step_by(30) {
            let x = rect.left() + i as f32;
            let alpha = 15 + ((time + i as f32 * 0.1).cos() * 5.0) as u8;
            painter.line_segment(
                [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                egui::Stroke::new(0.5, egui::Color32::from_rgba_premultiplied(0, 255, 255, alpha))
            );
        }
    }
    
    // Painel de controle com stats
    fn render_control_panel(&self, ui: &mut egui::Ui) {
        let time = self.animation_time.elapsed().as_secs_f32();
        
        egui::Frame::none()
            .fill(egui::Color32::from_rgb(10, 25, 35))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 180, 200)))
            .inner_margin(egui::Margin::same(15.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Stats do sistema
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("[ SYSTEM STATUS ]")
                                .size(12.0)
                                .color(egui::Color32::from_rgb(0, 255, 255))
                                .family(egui::FontFamily::Monospace)
                        );
                        
                        ui.add_space(8.0);
                        
                        if let Some(results) = self.scan_results.lock().unwrap().as_ref() {
                            let total_size_gb = results.total_size() as f64 / (1024.0 * 1024.0 * 1024.0);
                            
                            ui.label(
                                egui::RichText::new(format!("{:.2} GB", total_size_gb))
                                    .size(32.0)
                                    .color(egui::Color32::from_rgb(255, 50, 80))
                                    .family(egui::FontFamily::Monospace)
                            );
                            
                            ui.label(
                                egui::RichText::new(format!("{} FILES DETECTED", results.total_items()))
                                    .size(11.0)
                                    .color(egui::Color32::from_rgb(100, 200, 220))
                                    .family(egui::FontFamily::Monospace)
                            );
                        } else {
                            ui.label(
                                egui::RichText::new("-- GB")
                                    .size(32.0)
                                    .color(egui::Color32::from_rgb(100, 150, 170))
                                    .family(egui::FontFamily::Monospace)
                            );
                            
                            ui.label(
                                egui::RichText::new("AWAITING SCAN")
                                    .size(11.0)
                                    .color(egui::Color32::from_rgb(100, 150, 170))
                                    .family(egui::FontFamily::Monospace)
                            );
                        }
                    });
                    
                    ui.add_space(40.0);
                    
                    // Indicador circular animado
                    let (response, painter) = ui.allocate_painter(
                        egui::vec2(80.0, 80.0),
                        egui::Sense::hover()
                    );
                    
                    let center = response.rect.center();
                    let radius = 35.0;
                    
                    // Círculos concêntricos
                    for i in 0..3 {
                        let r = radius - (i as f32 * 10.0);
                        let alpha = ((time * 2.0 + i as f32).sin() * 100.0 + 155.0) as u8;
                        painter.circle_stroke(
                            center,
                            r,
                            egui::Stroke::new(1.5, egui::Color32::from_rgba_premultiplied(0, 255, 255, alpha))
                        );
                    }
                    
                    // Linhas radiais
                    for i in 0..8 {
                        let angle = (time + i as f32 * 0.785) % (2.0 * std::f32::consts::PI);
                        let x = center.x + angle.cos() * radius;
                        let y = center.y + angle.sin() * radius;
                        painter.line_segment(
                            [center, egui::pos2(x, y)],
                            egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(0, 255, 255, 100))
                        );
                    }
                });
            });
    }
}

impl eframe::App for PCCleanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();
        apply_cyberpunk_theme(ctx);
        
        // Efeito de grid de fundo
        self.render_tech_grid(ctx);
        
        self.render_status_bar(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            self.render_header(ui);
            ui.add_space(30.0);
            
            // Painel de controle
            self.render_control_panel(ui);
            
            ui.add_space(20.0);
            
            self.render_action_buttons(ui);
            ui.add_space(30.0);
            
            self.render_results(ui);
        });

        self.render_confirmation_dialog(ctx);
        self.render_cleaning_progress(ctx);
    }
}
