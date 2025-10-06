use crate::domain::repositories::CleanerRepository;
use crate::domain::entities::CleanableCategory;
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Implementação concreta do CleanerRepository.
/// Realiza operações de limpeza no sistema de arquivos e serviços.
pub struct FileSystemCleanerRepository;

impl FileSystemCleanerRepository {
    pub fn new() -> Self {
        Self
    }

    /// Limpa um arquivo individual.
    fn clean_file(&self, path: &Path) -> Result<()> {
        if path.exists() && path.is_file() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    /// Limpa um diretório completo.
    fn clean_directory(&self, path: &Path) -> Result<()> {
        if path.exists() && path.is_dir() {
            fs::remove_dir_all(path)?;
        }
        Ok(())
    }

    /// Limpa imagens Docker usando comando CLI.
    fn clean_docker_images(&self) -> Result<usize> {
        let output = Command::new("docker")
            .args(&["image", "prune", "-f"])
            .output()?;

        if output.status.success() {
            // Parse do output para contar itens removidos
            let _stdout = String::from_utf8_lossy(&output.stdout);
            // Docker retorna algo como "Total reclaimed space: X"
            Ok(1) // Simplificado - em produção parsearia o output
        } else {
            Ok(0)
        }
    }

    /// Limpa volumes Docker usando comando CLI.
    fn clean_docker_volumes(&self) -> Result<usize> {
        let output = Command::new("docker")
            .args(&["volume", "prune", "-f"])
            .output()?;

        if output.status.success() {
            Ok(1)
        } else {
            Ok(0)
        }
    }

    /// Determina o tipo de limpeza baseado no nome da categoria.
    fn get_cleanup_strategy(&self, category_name: &str) -> CleanupStrategy {
        if category_name.contains("Docker") {
            if category_name.contains("Imagens") {
                CleanupStrategy::DockerImages
            } else {
                CleanupStrategy::DockerVolumes
            }
        } else {
            CleanupStrategy::FileSystem
        }
    }
}

#[derive(Debug, PartialEq)]
enum CleanupStrategy {
    FileSystem,
    DockerImages,
    DockerVolumes,
}

impl CleanerRepository for FileSystemCleanerRepository {
    fn clean_category(&self, category: &CleanableCategory) -> Result<usize> {
        let strategy = self.get_cleanup_strategy(category.name());
        let mut cleaned_count = 0;

        match strategy {
            CleanupStrategy::DockerImages => {
                cleaned_count += self.clean_docker_images()?;
            }
            CleanupStrategy::DockerVolumes => {
                cleaned_count += self.clean_docker_volumes()?;
            }
            CleanupStrategy::FileSystem => {
                for item in category.items() {
                    let path = Path::new(item.path());
                    
                    let result = if path.is_file() {
                        self.clean_file(path)
                    } else if path.is_dir() {
                        self.clean_directory(path)
                    } else {
                        continue;
                    };

                    if result.is_ok() {
                        cleaned_count += 1;
                    }
                }
            }
        }

        Ok(cleaned_count)
    }

    fn can_clean(&self, category: &CleanableCategory) -> bool {
        // Verificações de segurança
        !category.is_empty() && !category.name().is_empty()
    }
}

impl Default for FileSystemCleanerRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::CleanableItem;

    #[test]
    fn should_identify_docker_strategy() {
        let repo = FileSystemCleanerRepository::new();
        assert_eq!(
            repo.get_cleanup_strategy("🐳 Imagens Docker"),
            CleanupStrategy::DockerImages
        );
    }

    #[test]
    fn should_validate_cleanable_category() {
        let repo = FileSystemCleanerRepository::new();
        let mut category = CleanableCategory::new("Test".to_string());
        
        assert!(!repo.can_clean(&category)); // Empty category
        
        category.add_item(CleanableItem::new("/tmp/test".to_string(), 100));
        assert!(repo.can_clean(&category)); // Non-empty category
    }
}
