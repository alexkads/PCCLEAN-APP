use std::fs;
use std::path::Path;
use std::process::Command;
use crate::scanner::ScanResult;
use anyhow::Result;

pub struct Cleaner;

impl Cleaner {
    pub fn new() -> Self {
        Self
    }

    pub fn clean_selected(&self, results: &ScanResult, selected: &[bool]) -> Result<()> {
        if selected.len() < 5 {
            return Ok(());
        }

        // Limpar arquivos de log
        if selected[0] {
            self.clean_log_files(&results.log_files)?;
        }

        // Limpar arquivos temporários
        if selected[1] {
            self.clean_temp_files(&results.temp_files)?;
        }

        // Limpar imagens Docker
        if selected[2] {
            self.clean_docker_images(&results.docker_images)?;
        }

        // Limpar volumes Docker
        if selected[3] {
            self.clean_docker_volumes(&results.docker_volumes)?;
        }

        // Limpar pacotes de desenvolvimento
        if selected[4] {
            self.clean_dev_packages(&results.dev_packages)?;
        }

        Ok(())
    }

    fn clean_log_files(&self, category: &crate::scanner::CleanableCategory) -> Result<()> {
        for item in &category.items {
            let path = Path::new(&item.path);
            if path.exists() && path.is_file() {
                // Tentar remover o arquivo
                let _ = fs::remove_file(path);
            }
        }
        Ok(())
    }

    fn clean_temp_files(&self, category: &crate::scanner::CleanableCategory) -> Result<()> {
        for item in &category.items {
            let path = Path::new(&item.path);
            if path.exists() {
                if path.is_file() {
                    let _ = fs::remove_file(path);
                } else if path.is_dir() {
                    let _ = fs::remove_dir_all(path);
                }
            }
        }
        Ok(())
    }

    fn clean_docker_images(&self, _category: &crate::scanner::CleanableCategory) -> Result<()> {
        // Executar comando para remover imagens dangling
        let _ = Command::new("docker")
            .args(&["image", "prune", "-f"])
            .output();

        Ok(())
    }

    fn clean_docker_volumes(&self, _category: &crate::scanner::CleanableCategory) -> Result<()> {
        // Executar comando para remover volumes dangling
        let _ = Command::new("docker")
            .args(&["volume", "prune", "-f"])
            .output();

        Ok(())
    }

    fn clean_dev_packages(&self, category: &crate::scanner::CleanableCategory) -> Result<()> {
        for item in &category.items {
            let path = Path::new(&item.path);
            if path.exists() && path.is_dir() {
                // Remover diretórios node_modules e caches
                let _ = fs::remove_dir_all(path);
            }
        }
        Ok(())
    }
}
