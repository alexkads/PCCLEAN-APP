use std::sync::{Arc, Mutex};
use slint::{ModelRc, VecModel, SharedString, Model};

// Incluir o código UI gerado pelo Slint
slint::include_modules!();

use crate::application::{ScanSystemUseCase, CleanSelectedCategoriesUseCase};
use crate::domain::entities::ScanResult;
use crate::infrastructure::{FileSystemScannerRepository, FileSystemCleanerRepository};

/// Executa a aplicação Slint
pub fn run_app() -> Result<(), slint::PlatformError> {
    let app = SlintApp::new();
    app.run()
}

/// Aplicação principal usando Slint
pub struct SlintApp {
    window: AppWindow,
    scan_use_case: Arc<ScanSystemUseCase>,
    clean_use_case: Arc<CleanSelectedCategoriesUseCase>,
    scan_results: Arc<Mutex<Option<ScanResult>>>,
}

impl SlintApp {
    pub fn new() -> Self {
        // Criar window
        let window = AppWindow::new().expect("Failed to create window");
        
        // Injeção de dependências
        let scanner_repo = Arc::new(FileSystemScannerRepository::new());
        let cleaner_repo = Arc::new(FileSystemCleanerRepository::new());
        
        let scan_use_case = Arc::new(ScanSystemUseCase::new(scanner_repo));
        let clean_use_case = Arc::new(CleanSelectedCategoriesUseCase::new(cleaner_repo));
        
        let scan_results = Arc::new(Mutex::new(None));
        
        // Setup callbacks
        let mut app = Self {
            window,
            scan_use_case,
            clean_use_case,
            scan_results,
        };
        
        app.setup_callbacks();
        app
    }
    
    fn setup_callbacks(&mut self) {
        let window_weak = self.window.as_weak();
        let scan_use_case = Arc::clone(&self.scan_use_case);
        let scan_results = Arc::clone(&self.scan_results);
        
        // Callback: Start Scan
        self.window.on_start_scan(move || {
            if let Some(window) = window_weak.upgrade() {
                window.set_is_scanning(true);
                window.set_has_results(false);
                
                let use_case = Arc::clone(&scan_use_case);
                let results = Arc::clone(&scan_results);
                let window_clone = window_weak.clone();
                
                std::thread::spawn(move || {
                    if let Ok(result) = use_case.execute() {
                        // Converter para dados da UI
                        let total_size = result.total_size();
                        let total_gb = total_size as f64 / (1024.0 * 1024.0 * 1024.0);
                        let total_items = result.total_items();
                        
                        let categories: Vec<CategoryData> = result.categories()
                            .iter()
                            .map(|cat| {
                                let size_gb = cat.total_size() as f64 / (1024.0 * 1024.0 * 1024.0);
                                let icon = match cat.name().to_lowercase().as_str() {
                                    name if name.contains("log") => "📋",
                                    name if name.contains("cache") => "💾",
                                    name if name.contains("download") => "⬇️",
                                    name if name.contains("temp") => "🗂️",
                                    name if name.contains("docker") => "🐳",
                                    _ => "📁",
                                };
                                
                                CategoryData {
                                    name: SharedString::from(cat.name()),
                                    items_count: cat.items().len() as i32,
                                    total_size: SharedString::from(format!("{:.2} GB", size_gb)),
                                    selected: false,
                                    icon: SharedString::from(icon),
                                }
                            })
                            .collect();
                        
                        // Atualizar UI no thread principal
                        slint::invoke_from_event_loop(move || {
                            if let Some(window) = window_clone.upgrade() {
                                window.set_is_scanning(false);
                                window.set_has_results(true);
                                window.set_total_size_display(SharedString::from(format!("{:.2}", total_gb)));
                                window.set_total_items(total_items as i32);
                                
                                // Criar model para as categorias
                                let cat_model = std::rc::Rc::new(VecModel::from(categories));
                                window.set_categories(ModelRc::from(cat_model.clone()));
                            }
                        }).ok();
                        
                        *results.lock().unwrap() = Some(result);
                    } else {
                        slint::invoke_from_event_loop(move || {
                            if let Some(window) = window_clone.upgrade() {
                                window.set_is_scanning(false);
                            }
                        }).ok();
                    }
                });
            }
        });
        
        // Callback: Category Selection Changed
        let window_weak = self.window.as_weak();
        self.window.on_category_selection_changed(move |index, checked| {
            if let Some(window) = window_weak.upgrade() {
                let categories = window.get_categories();
                let model = VecModel::from_slice(&categories.iter().collect::<Vec<_>>());
                
                if (index as usize) < model.row_count() {
                    if let Some(mut category) = model.row_data(index as usize) {
                        category.selected = checked;
                        model.set_row_data(index as usize, category);
                        window.set_categories(ModelRc::new(model));
                    }
                }
            }
        });
        
        // Callback: Confirm Clean
        let window_weak = self.window.as_weak();
        let clean_use_case = Arc::clone(&self.clean_use_case);
        let scan_results = Arc::clone(&self.scan_results);
        
        self.window.on_confirm_clean(move || {
            if let Some(window) = window_weak.upgrade() {
                let categories = window.get_categories();
                
                // Coletar índices selecionados
                let mut selected_indices = Vec::new();
                for (i, category) in categories.iter().enumerate() {
                    if category.selected {
                        selected_indices.push(i);
                    }
                }
                
                if selected_indices.is_empty() {
                    return;
                }
                
                window.set_is_cleaning(true);
                
                let use_case = Arc::clone(&clean_use_case);
                let results = Arc::clone(&scan_results);
                let window_clone = window_weak.clone();
                
                std::thread::spawn(move || {
                    if let Some(scan_result) = results.lock().unwrap().as_ref() {
                        match use_case.execute(scan_result, &selected_indices) {
                            Ok(_cleaned_count) => {
                                slint::invoke_from_event_loop(move || {
                                    if let Some(window) = window_clone.upgrade() {
                                        window.set_is_cleaning(false);
                                        window.set_has_results(false);
                                        window.set_categories(ModelRc::new(VecModel::default()));
                                    }
                                }).ok();
                            }
                            Err(_) => {
                                slint::invoke_from_event_loop(move || {
                                    if let Some(window) = window_clone.upgrade() {
                                        window.set_is_cleaning(false);
                                    }
                                }).ok();
                            }
                        }
                    }
                });
            }
        });
        
        // Callback: Cancel Clean
        self.window.on_cancel_clean(move || {
            // Nada a fazer, dialog já fecha automaticamente
        });
    }
    
    pub fn run(self) -> Result<(), slint::PlatformError> {
        self.window.run()
    }
}
