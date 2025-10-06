use std::path::Path;
use walkdir::WalkDir;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct CleanableItem {
    pub path: String,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct CleanableCategory {
    pub items: Vec<CleanableItem>,
    pub total_size: u64,
}

impl CleanableCategory {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            total_size: 0,
        }
    }

    pub fn add_item(&mut self, path: String, size: u64) {
        self.items.push(CleanableItem { path, size });
        self.total_size += size;
    }
}

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub log_files: CleanableCategory,
    pub temp_files: CleanableCategory,
    pub docker_images: CleanableCategory,
    pub docker_volumes: CleanableCategory,
    pub dev_packages: CleanableCategory,
}

impl ScanResult {
    pub fn new() -> Self {
        Self {
            log_files: CleanableCategory::new(),
            temp_files: CleanableCategory::new(),
            docker_images: CleanableCategory::new(),
            docker_volumes: CleanableCategory::new(),
            dev_packages: CleanableCategory::new(),
        }
    }

    pub fn total_size(&self) -> u64 {
        self.log_files.total_size
            + self.temp_files.total_size
            + self.docker_images.total_size
            + self.docker_volumes.total_size
            + self.dev_packages.total_size
    }
}

pub struct Scanner;

impl Scanner {
    pub fn new() -> Self {
        Self
    }

    pub fn scan_all(&self) -> ScanResult {
        let mut result = ScanResult::new();

        // Escanear arquivos de log
        self.scan_log_files(&mut result.log_files);

        // Escanear arquivos temporários
        self.scan_temp_files(&mut result.temp_files);

        // Escanear Docker
        self.scan_docker_images(&mut result.docker_images);
        self.scan_docker_volumes(&mut result.docker_volumes);

        // Escanear pacotes de desenvolvimento
        self.scan_dev_packages(&mut result.dev_packages);

        result
    }

    fn scan_log_files(&self, category: &mut CleanableCategory) {
        // Diretórios comuns de log
        let home = std::env::var("HOME").unwrap_or_default();
        let user_logs = format!("{}/Library/Logs", home);
        
        let log_dirs = vec![
            "/var/log",
            "/tmp",
            user_logs.as_str(),
        ];

        for dir in log_dirs {
            let entries: Vec<_> = WalkDir::new(dir)
                .max_depth(3)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
                .filter(|e| {
                    let path = e.path();
                    path.extension().map_or(false, |ext| {
                        ext == "log" || ext == "LOG"
                    }) || path.to_string_lossy().contains(".log")
                })
                .collect();

            for entry in entries {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.len() > 0 {
                        category.add_item(
                            entry.path().to_string_lossy().to_string(),
                            metadata.len(),
                        );
                    }
                }
            }
        }
    }

    fn scan_temp_files(&self, category: &mut CleanableCategory) {
        let home = std::env::var("HOME").unwrap_or_default();
        let user_caches = format!("{}/Library/Caches", home);
        
        let temp_dirs = vec![
            "/tmp",
            "/var/tmp",
            user_caches.as_str(),
        ];

        for dir in temp_dirs {
            let entries: Vec<_> = WalkDir::new(dir)
                .max_depth(2)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
                .collect();

            for entry in entries.iter().take(100) { // Limitar para performance
                if let Ok(metadata) = entry.metadata() {
                    if metadata.len() > 0 {
                        category.add_item(
                            entry.path().to_string_lossy().to_string(),
                            metadata.len(),
                        );
                    }
                }
            }
        }
    }

    fn scan_docker_images(&self, category: &mut CleanableCategory) {
        // Tentar obter imagens Docker não utilizadas
        if let Ok(output) = Command::new("docker")
            .args(&["images", "--filter", "dangling=true", "-q", "--no-trunc"])
            .output()
        {
            if output.status.success() {
                let image_ids = String::from_utf8_lossy(&output.stdout);
                
                for image_id in image_ids.lines() {
                    if image_id.trim().is_empty() {
                        continue;
                    }

                    // Obter tamanho da imagem
                    if let Ok(size_output) = Command::new("docker")
                        .args(&["inspect", "-f", "{{.Size}}", image_id.trim()])
                        .output()
                    {
                        if let Ok(size_str) = String::from_utf8(size_output.stdout) {
                            if let Ok(size) = size_str.trim().parse::<u64>() {
                                category.add_item(
                                    format!("Docker Image: {}", &image_id[..12]),
                                    size,
                                );
                            }
                        }
                    }
                }
            }
        }

        // Se não houver imagens dangling, adicionar exemplo
        if category.items.is_empty() {
            category.add_item(
                "Nenhuma imagem Docker não utilizada encontrada".to_string(),
                0,
            );
        }
    }

    fn scan_docker_volumes(&self, category: &mut CleanableCategory) {
        // Tentar obter volumes Docker não utilizados
        if let Ok(output) = Command::new("docker")
            .args(&["volume", "ls", "-qf", "dangling=true"])
            .output()
        {
            if output.status.success() {
                let volume_names = String::from_utf8_lossy(&output.stdout);
                
                for volume_name in volume_names.lines() {
                    if volume_name.trim().is_empty() {
                        continue;
                    }

                    // Estimar tamanho (Docker não fornece tamanho direto para volumes)
                    let estimated_size = 100_000_000; // 100MB estimado
                    
                    category.add_item(
                        format!("Docker Volume: {}", volume_name.trim()),
                        estimated_size,
                    );
                }
            }
        }

        // Se não houver volumes, adicionar exemplo
        if category.items.is_empty() {
            category.add_item(
                "Nenhum volume Docker não utilizado encontrado".to_string(),
                0,
            );
        }
    }

    fn scan_dev_packages(&self, category: &mut CleanableCategory) {
        let home = std::env::var("HOME").unwrap_or_default();
        
        // Procurar por node_modules
        let common_dev_dirs = vec![
            format!("{}/Projects", home),
            format!("{}/Documents", home),
            format!("{}/Desktop", home),
        ];

        for base_dir in common_dev_dirs {
            if !Path::new(&base_dir).exists() {
                continue;
            }

            // Procurar node_modules
            let entries: Vec<_> = WalkDir::new(&base_dir)
                .max_depth(4)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path().is_dir() && 
                    e.path().file_name().map_or(false, |n| n == "node_modules")
                })
                .collect();

            for entry in entries.iter().take(20) {
                if let Ok(size) = Self::get_dir_size(entry.path()) {
                    if size > 0 {
                        category.add_item(
                            entry.path().to_string_lossy().to_string(),
                            size,
                        );
                    }
                }
            }
        }

        // Adicionar cache do NPM
        let npm_cache = format!("{}/.npm", home);
        if let Ok(size) = Self::get_dir_size(Path::new(&npm_cache)) {
            if size > 0 {
                category.add_item(npm_cache, size);
            }
        }

        // Adicionar cache do Cargo
        let cargo_cache = format!("{}/.cargo/registry", home);
        if let Ok(size) = Self::get_dir_size(Path::new(&cargo_cache)) {
            if size > 0 {
                category.add_item(cargo_cache, size);
            }
        }

        // Se não houver pacotes, adicionar mensagem
        if category.items.is_empty() {
            category.add_item(
                "Nenhum pacote de desenvolvimento encontrado".to_string(),
                0,
            );
        }
    }

    fn get_dir_size(path: &Path) -> Result<u64, std::io::Error> {
        let mut size = 0u64;
        
        if path.is_dir() {
            for entry in WalkDir::new(path)
                .max_depth(10)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.path().is_file() {
                    if let Ok(metadata) = entry.metadata() {
                        size += metadata.len();
                    }
                }
            }
        }
        
        Ok(size)
    }
}
