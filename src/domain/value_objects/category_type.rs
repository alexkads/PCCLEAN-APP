/// Value Object que representa os tipos de categorias disponíveis.
/// Tipos imutáveis que garantem consistência no domínio.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CategoryType {
    LogFiles,
    TemporaryFiles,
    DockerImages,
    DockerVolumes,
    DevelopmentPackages,
}

impl CategoryType {
    /// Retorna o nome amigável da categoria.
    pub fn display_name(&self) -> &'static str {
        match self {
            CategoryType::LogFiles => "📄 Arquivos de Log",
            CategoryType::TemporaryFiles => "🗂️ Arquivos Temporários",
            CategoryType::DockerImages => "🐳 Imagens Docker",
            CategoryType::DockerVolumes => "💾 Volumes Docker",
            CategoryType::DevelopmentPackages => "📦 Pacotes de Desenvolvimento",
        }
    }

    /// Retorna o identificador interno da categoria.
    pub fn identifier(&self) -> &'static str {
        match self {
            CategoryType::LogFiles => "log_files",
            CategoryType::TemporaryFiles => "temp_files",
            CategoryType::DockerImages => "docker_images",
            CategoryType::DockerVolumes => "docker_volumes",
            CategoryType::DevelopmentPackages => "dev_packages",
        }
    }

    /// Retorna todas as categorias disponíveis.
    pub fn all() -> Vec<CategoryType> {
        vec![
            CategoryType::LogFiles,
            CategoryType::TemporaryFiles,
            CategoryType::DockerImages,
            CategoryType::DockerVolumes,
            CategoryType::DevelopmentPackages,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_display_name() {
        assert_eq!(CategoryType::LogFiles.display_name(), "📄 Arquivos de Log");
    }

    #[test]
    fn should_return_all_categories() {
        let all = CategoryType::all();
        assert_eq!(all.len(), 5);
    }
}
