use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use crate::scanner::{ScanResult, Scanner, CleanableCategory};
use crate::cleaner::Cleaner;

pub struct PCCleanApp {
    scanner: Arc<Mutex<Scanner>>,
    cleaner: Arc<Mutex<Cleaner>>,
    scan_results: Arc<Mutex<Option<ScanResult>>>,
    is_scanning: bool,
    is_cleaning: bool,
    selected_categories: Vec<bool>,
    animation_time: Instant,
    scan_progress: f32,
    clean_progress: f32,
    show_confirmation: bool,
    log_messages: Vec<String>,
}

impl PCCleanApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            scanner: Arc::new(Mutex::new(Scanner::new())),
            cleaner: Arc::new(Mutex::new(Cleaner::new())),
            scan_results: Arc::new(Mutex::new(None)),
            is_scanning: false,
            is_cleaning: false,
            selected_categories: vec![false; 5],
            animation_time: Instant::now(),
            scan_progress: 0.0,
            clean_progress: 0.0,
            show_confirmation: false,
            log_messages: Vec::new(),
        }
    }

    fn start_scan(&mut self) {
        if self.is_scanning {
            return;
        }

        self.is_scanning = true;
        self.scan_progress = 0.0;
        self.log_messages.clear();
        self.log_messages.push("🔍 Iniciando varredura do sistema...".to_string());

        let scanner = Arc::clone(&self.scanner);
        let results = Arc::clone(&self.scan_results);

        std::thread::spawn(move || {
            let result = scanner.lock().unwrap().scan_all();
            *results.lock().unwrap() = Some(result);
        });
    }

    fn start_cleaning(&mut self) {
        if self.is_cleaning || !self.show_confirmation {
            return;
        }

        self.is_cleaning = true;
        self.clean_progress = 0.0;
        self.show_confirmation = false;
        self.log_messages.push("🧹 Iniciando limpeza...".to_string());

        let cleaner = Arc::clone(&self.cleaner);
        let selected = self.selected_categories.clone();
        let results = Arc::clone(&self.scan_results);

        std::thread::spawn(move || {
            if let Some(scan_result) = results.lock().unwrap().as_ref() {
                let _ = cleaner.lock().unwrap().clean_selected(scan_result, &selected);
            }
        });
    }
}

impl eframe::App for PCCleanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Animação contínua
        ctx.request_repaint();
        let time = self.animation_time.elapsed().as_secs_f32();

        // Aplicar tema cyberpunk
        apply_cyberpunk_theme(ctx);

        // Painel superior (header)
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                
                // Título com efeito neon animado
                let glow = (time * 2.0).sin() * 0.3 + 0.7;
                ui.heading(
                    egui::RichText::new("⚡ PCCLEAN")
                        .size(36.0)
                        .color(egui::Color32::from_rgb(
                            (0.0 * glow * 255.0) as u8,
                            (1.0 * glow * 255.0) as u8,
                            (1.0 * glow * 255.0) as u8,
                        ))
                );
                
                ui.label(
                    egui::RichText::new("Cyberpunk Disk Cleaner v0.1")
                        .size(14.0)
                        .color(egui::Color32::from_rgb(138, 43, 226))
                );
                
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
            });
            ui.add_space(10.0);
        });

        // Painel inferior (status bar)
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                
                let status_color = if self.is_scanning || self.is_cleaning {
                    egui::Color32::from_rgb(255, 165, 0) // Laranja
                } else {
                    egui::Color32::from_rgb(0, 255, 0) // Verde
                };
                
                ui.label(
                    egui::RichText::new("●")
                        .size(12.0)
                        .color(status_color)
                );
                
                if let Some(results) = self.scan_results.lock().unwrap().as_ref() {
                    let total_size = results.total_size();
                    ui.label(
                        egui::RichText::new(format!("Total encontrado: {}", format_bytes(total_size)))
                            .color(egui::Color32::from_rgb(138, 43, 226))
                    );
                } else {
                    ui.label(
                        egui::RichText::new("Pronto para varredura")
                            .color(egui::Color32::GRAY)
                    );
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(20.0);
                    ui.label(
                        egui::RichText::new(&format!("FPS: {:.0}", ctx.input(|i| 1.0 / i.stable_dt)))
                            .color(egui::Color32::from_rgb(100, 100, 100))
                    );
                });
            });
            ui.add_space(5.0);
        });

        // Painel central
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(20.0);

            // Botão de scan com efeito neon
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                
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

                ui.add_space(20.0);

                // Botão de limpeza
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
            });

            ui.add_space(20.0);

            // Área de resultados
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(results) = self.scan_results.lock().unwrap().as_ref() {
                    self.is_scanning = false;
                    self.scan_progress = 1.0;

                    ui.add_space(10.0);

                    // Renderizar cada categoria
                    render_category(ui, "📄 Arquivos de Log", &results.log_files, 0, &mut self.selected_categories, time);
                    render_category(ui, "🗂️ Arquivos Temporários", &results.temp_files, 1, &mut self.selected_categories, time);
                    render_category(ui, "🐳 Imagens Docker", &results.docker_images, 2, &mut self.selected_categories, time);
                    render_category(ui, "💾 Volumes Docker", &results.docker_volumes, 3, &mut self.selected_categories, time);
                    render_category(ui, "📦 Pacotes Temporários", &results.dev_packages, 4, &mut self.selected_categories, time);

                } else if !self.is_scanning {
                    // Estado inicial - arte ASCII cyberpunk
                    ui.vertical_centered(|ui| {
                        ui.add_space(50.0);
                        ui.label(
                            egui::RichText::new(
                                "╔═══════════════════════════════════════╗\n\
                                 ║   ⚡ PCCLEAN CYBERPUNK EDITION ⚡   ║\n\
                                 ║                                       ║\n\
                                 ║   Clique em INICIAR VARREDURA         ║\n\
                                 ║   para começar a análise              ║\n\
                                 ╚═══════════════════════════════════════╝"
                            )
                            .size(16.0)
                            .color(egui::Color32::from_rgb(0, 255, 255))
                            .family(egui::FontFamily::Monospace)
                        );
                    });
                }
            });
        });

        // Diálogo de confirmação
        if self.show_confirmation {
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
                        if ui.button(
                            egui::RichText::new("✓ Confirmar")
                                .size(14.0)
                        ).clicked() {
                            self.start_cleaning();
                        }

                        if ui.button(
                            egui::RichText::new("✗ Cancelar")
                                .size(14.0)
                        ).clicked() {
                            self.show_confirmation = false;
                        }
                    });
                });
        }

        // Indicador de limpeza em progresso
        if self.is_cleaning {
            egui::Window::new("🧹 Limpeza em Progresso")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.add_space(10.0);
                    ui.spinner();
                    ui.label("Limpando arquivos...");
                    ui.add_space(10.0);
                    
                    // Verificar se terminou
                    if self.clean_progress >= 1.0 {
                        self.is_cleaning = false;
                        self.scan_results = Arc::new(Mutex::new(None));
                        self.selected_categories = vec![false; 5];
                    }
                });
            
            // Simular progresso
            self.clean_progress += 0.01;
        }
    }
}

fn render_category(
    ui: &mut egui::Ui,
    title: &str,
    category: &CleanableCategory,
    index: usize,
    selected: &mut [bool],
    time: f32,
) {
    ui.add_space(10.0);
    
    let frame_color = if selected[index] {
        egui::Color32::from_rgb(255, 0, 255) // Magenta quando selecionado
    } else {
        egui::Color32::from_rgb(138, 43, 226) // Roxo padrão
    };

    egui::Frame::none()
        .fill(egui::Color32::from_rgba_premultiplied(10, 10, 30, 200))
        .stroke(egui::Stroke::new(2.0, frame_color))
        .inner_margin(egui::Margin::same(15.0))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                // Checkbox com estilo
                if ui.checkbox(&mut selected[index], "").changed() {
                    // Feedback visual
                }

                ui.label(
                    egui::RichText::new(title)
                        .size(18.0)
                        .color(egui::Color32::from_rgb(0, 255, 255))
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let size_color = if category.total_size > 1_000_000_000 {
                        egui::Color32::from_rgb(255, 0, 0) // Vermelho para > 1GB
                    } else if category.total_size > 100_000_000 {
                        egui::Color32::from_rgb(255, 165, 0) // Laranja para > 100MB
                    } else {
                        egui::Color32::from_rgb(0, 255, 0) // Verde
                    };

                    ui.label(
                        egui::RichText::new(format_bytes(category.total_size))
                            .size(16.0)
                            .color(size_color)
                    );

                    ui.label(
                        egui::RichText::new(format!("({} itens)", category.items.len()))
                            .color(egui::Color32::GRAY)
                    );
                });
            });

            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);

            // Mostrar alguns itens como exemplo
            for (i, item) in category.items.iter().take(3).enumerate() {
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    
                    // Efeito piscante para o primeiro item
                    let alpha = if i == 0 {
                        ((time * 3.0).sin() * 0.5 + 0.5) * 255.0
                    } else {
                        200.0
                    };
                    
                    ui.label(
                        egui::RichText::new(&item.path)
                            .color(egui::Color32::from_rgba_premultiplied(150, 150, 150, alpha as u8))
                            .family(egui::FontFamily::Monospace)
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new(format_bytes(item.size))
                                .color(egui::Color32::GRAY)
                        );
                    });
                });
            }

            if category.items.len() > 3 {
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.label(
                        egui::RichText::new(format!("... e mais {} itens", category.items.len() - 3))
                            .color(egui::Color32::DARK_GRAY)
                            .italics()
                    );
                });
            }
        });
}

fn apply_cyberpunk_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    
    // Cores neon cyberpunk
    style.visuals.dark_mode = true;
    style.visuals.override_text_color = Some(egui::Color32::from_rgb(200, 200, 255));
    style.visuals.panel_fill = egui::Color32::from_rgb(10, 10, 25);
    style.visuals.window_fill = egui::Color32::from_rgba_premultiplied(15, 15, 35, 240);
    style.visuals.extreme_bg_color = egui::Color32::from_rgb(5, 5, 15);
    
    // Widgets com brilho neon
    style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(20, 20, 40);
    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 30, 50);
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(50, 50, 80);
    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(80, 40, 120);
    
    // Bordas brilhantes
    style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(138, 43, 226));
    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 255, 255));
    style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 0, 255));
    style.visuals.widgets.active.bg_stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 255, 0));
    
    style.visuals.selection.bg_fill = egui::Color32::from_rgba_premultiplied(138, 43, 226, 100);
    style.visuals.window_shadow.color = egui::Color32::from_rgba_premultiplied(138, 43, 226, 50);
    
    ctx.set_style(style);
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}
