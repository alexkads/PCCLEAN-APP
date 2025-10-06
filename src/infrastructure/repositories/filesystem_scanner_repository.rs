use crate::domain::repositories::ScannerRepository;
use crate::domain::entities::{ScanResult, CleanableCategory, CleanableItem};
use crate::domain::value_objects::CategoryType;
use anyhow::Result;
use walkdir::WalkDir;
use std::path::Path;
use std::process::Command;

/// Implementação concreta do ScannerRepository.
/// Realiza varreduras no sistema de arquivos e serviços.
pub struct FileSystemScannerRepository;

impl FileSystemScannerRepository {
    pub fn new() -> Self {
        Self
    }

    /// Escaneia arquivos de log do sistema.
    fn scan_log_files(&self) -> Result<Vec<CleanableItem>> {
        let mut items = Vec::new();
        let home = std::env::var("HOME").unwrap_or_default();
        let user_logs = format!("{}/Library/Logs", home);
        
        let log_dirs = vec!["/var/log", "/tmp", user_logs.as_str()];

        for dir in log_dirs {
            if !Path::new(dir).exists() {
                continue;
            }

            for entry in WalkDir::new(dir)
                .max_depth(3)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
                .filter(|e| Self::is_log_file(e.path()))
            {
                if let Ok(metadata) = entry.metadata() {
                    let size = metadata.len();
                    if size > 0 {
                        items.push(CleanableItem::new(
                            entry.path().to_string_lossy().to_string(),
                            size,
                        ));
                    }
                }
            }
        }

        Ok(items)
    }

    fn is_log_file(path: &Path) -> bool {
        path.extension()
            .map_or(false, |ext| ext == "log" || ext == "LOG")
            || path.to_string_lossy().contains(".log")
    }

    /// Escaneia arquivos temporários.
    fn scan_temp_files(&self) -> Result<Vec<CleanableItem>> {
        let mut items = Vec::new();
        let home = std::env::var("HOME").unwrap_or_default();
        let user_caches = format!("{}/Library/Caches", home);
        
        let temp_dirs = vec!["/tmp", "/var/tmp", user_caches.as_str()];

        for dir in temp_dirs {
            if !Path::new(dir).exists() {
                continue;
            }

            let entries: Vec<_> = WalkDir::new(dir)
                .max_depth(2)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
                .take(100) // Limitar para performance
                .collect();

            for entry in entries {
                if let Ok(metadata) = entry.metadata() {
                    let size = metadata.len();
                    if size > 0 {
                        items.push(CleanableItem::new(
                            entry.path().to_string_lossy().to_string(),
                            size,
                        ));
                    }
                }
            }
        }

        Ok(items)
    }

    /// Escaneia imagens Docker não utilizadas.
    fn scan_docker_images(&self) -> Result<Vec<CleanableItem>> {
        let mut items = Vec::new();

        let output = Command::new("docker")
            .args(&["images", "--filter", "dangling=true", "-q", "--no-trunc"])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let image_ids = String::from_utf8_lossy(&output.stdout);
                
                for image_id in image_ids.lines() {
                    let trimmed_id = image_id.trim();
                    if trimmed_id.is_empty() {
                        continue;
                    }

                    let size = Self::get_docker_image_size(trimmed_id).unwrap_or(0);
                    if size > 0 {
                        items.push(CleanableItem::new(
                            format!("Docker Image: {}", &trimmed_id[..12.min(trimmed_id.len())]),
                            size,
                        ));
                    }
                }
            }
        }

        // Se não houver itens, adicionar placeholder informativo
        if items.is_empty() {
            items.push(CleanableItem::new(
                "Nenhuma imagem Docker não utilizada encontrada".to_string(),
                0,
            ));
        }

        Ok(items)
    }

    fn get_docker_image_size(image_id: &str) -> Option<u64> {
        Command::new("docker")
            .args(&["inspect", "-f", "{{.Size}}", image_id])
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .and_then(|size_str| size_str.trim().parse::<u64>().ok())
    }

    /// Escaneia volumes Docker não utilizados.
    fn scan_docker_volumes(&self) -> Result<Vec<CleanableItem>> {
        let mut items = Vec::new();

        let output = Command::new("docker")
            .args(&["volume", "ls", "-qf", "dangling=true"])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let volume_names = String::from_utf8_lossy(&output.stdout);
                
                for volume_name in volume_names.lines() {
                    let trimmed_name = volume_name.trim();
                    if trimmed_name.is_empty() {
                        continue;
                    }

                    // Docker não fornece tamanho direto para volumes, estimamos
                    let estimated_size = 100_000_000; // 100MB
                    
                    items.push(CleanableItem::new(
                        format!("Docker Volume: {}", trimmed_name),
                        estimated_size,
                    ));
                }
            }
        }

        if items.is_empty() {
            items.push(CleanableItem::new(
                "Nenhum volume Docker não utilizado encontrado".to_string(),
                0,
            ));
        }

        Ok(items)
    }

    /// Escaneia pacotes de desenvolvimento (node_modules, cargo cache, etc).
    fn scan_dev_packages(&self) -> Result<Vec<CleanableItem>> {
        let mut items = Vec::new();
        let home = std::env::var("HOME").unwrap_or_default();
        
        // Procurar node_modules
        let common_dev_dirs = vec![
            format!("{}/Projects", home),
            format!("{}/Documents", home),
            format!("{}/Desktop", home),
        ];

        for base_dir in common_dev_dirs {
            if !Path::new(&base_dir).exists() {
                continue;
            }

            let entries: Vec<_> = WalkDir::new(&base_dir)
                .max_depth(4)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path().is_dir() && 
                    e.path().file_name().map_or(false, |n| n == "node_modules")
                })
                .take(20)
                .collect();

            for entry in entries {
                if let Ok(size) = Self::calculate_directory_size(entry.path()) {
                    if size > 0 {
                        items.push(CleanableItem::new(
                            entry.path().to_string_lossy().to_string(),
                            size,
                        ));
                    }
                }
            }
        }

        // Cache do NPM
        let npm_cache = format!("{}/.npm", home);
        if let Ok(size) = Self::calculate_directory_size(Path::new(&npm_cache)) {
            if size > 0 {
                items.push(CleanableItem::new(npm_cache, size));
            }
        }

        // Cache do Cargo
        let cargo_cache = format!("{}/.cargo/registry", home);
        if let Ok(size) = Self::calculate_directory_size(Path::new(&cargo_cache)) {
            if size > 0 {
                items.push(CleanableItem::new(cargo_cache, size));
            }
        }

        if items.is_empty() {
            items.push(CleanableItem::new(
                "Nenhum pacote de desenvolvimento encontrado".to_string(),
                0,
            ));
        }

        Ok(items)
    }

    fn calculate_directory_size(path: &Path) -> Result<u64> {
        let mut size = 0u64;
        
        if path.is_dir() {
            for entry in WalkDir::new(path)
                .max_depth(10)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
            {
                if let Ok(metadata) = entry.metadata() {
                    size += metadata.len();
                }
            }
        }
        
        Ok(size)
    }
}

impl ScannerRepository for FileSystemScannerRepository {
    fn scan_system(&self) -> Result<ScanResult> {
        let mut result = ScanResult::new();

        // Escanear cada categoria
        for category_type in CategoryType::all() {
            let items = self.scan_category(category_type)?;
            let mut category = CleanableCategory::new(category_type.display_name().to_string());
            
            for item in items {
                category.add_item(item);
            }
            
            result.add_category(category);
        }

        Ok(result)
    }

    fn scan_category(&self, category_type: CategoryType) -> Result<Vec<CleanableItem>> {
        match category_type {
            CategoryType::LogFiles => self.scan_log_files(),
            CategoryType::TemporaryFiles => self.scan_temp_files(),
            CategoryType::DockerImages => self.scan_docker_images(),
            CategoryType::DockerVolumes => self.scan_docker_volumes(),
            CategoryType::DevelopmentPackages => self.scan_dev_packages(),
        }
    }
}

impl Default for FileSystemScannerRepository {
    fn default() -> Self {
        Self::new()
    }
}
