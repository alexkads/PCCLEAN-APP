use crate::domain::repositories::CleanerRepository;
use crate::domain::entities::ScanResult;
use anyhow::{Result, anyhow};
use std::sync::Arc;

/// Caso de uso: Limpar categorias selecionadas.
/// Coordena a limpeza de múltiplas categorias de forma segura.
pub struct CleanSelectedCategoriesUseCase {
    cleaner_repository: Arc<dyn CleanerRepository>,
}

impl CleanSelectedCategoriesUseCase {
    pub fn new(cleaner_repository: Arc<dyn CleanerRepository>) -> Self {
        Self {
            cleaner_repository,
        }
    }

    /// Executa a limpeza das categorias selecionadas.
    /// 
    /// # Argumentos
    /// * `scan_result` - Resultado da varredura com as categorias
    /// * `selected_indices` - Índices das categorias selecionadas para limpeza
    /// 
    /// # Retorna
    /// Total de itens limpos com sucesso
    pub fn execute(&self, scan_result: &ScanResult, selected_indices: &[usize]) -> Result<usize> {
        let mut total_cleaned = 0;

        for &index in selected_indices {
            if let Some(category) = scan_result.categories().get(index) {
                if !category.is_empty() && self.cleaner_repository.can_clean(category) {
                    match self.cleaner_repository.clean_category(category) {
                        Ok(cleaned_count) => {
                            total_cleaned += cleaned_count;
                        }
                        Err(e) => {
                            // Log erro mas continua limpando outras categorias
                            eprintln!("Erro ao limpar categoria {}: {}", category.name(), e);
                        }
                    }
                }
            } else {
                return Err(anyhow!("Índice de categoria inválido: {}", index));
            }
        }

        Ok(total_cleaned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{CleanableCategory, CleanableItem};

    struct MockCleanerRepository;

    impl CleanerRepository for MockCleanerRepository {
        fn clean_category(&self, _category: &CleanableCategory) -> Result<usize> {
            Ok(5)
        }

        fn can_clean(&self, _category: &CleanableCategory) -> bool {
            true
        }
    }

    #[test]
    fn should_clean_selected_categories() {
        let repo = Arc::new(MockCleanerRepository);
        let use_case = CleanSelectedCategoriesUseCase::new(repo);
        
        let mut result = ScanResult::new();
        let mut cat = CleanableCategory::new("Test".to_string());
        cat.add_item(CleanableItem::new("/tmp/test.log".to_string(), 1024));
        result.add_category(cat);
        
        let cleaned = use_case.execute(&result, &[0]).unwrap();
        assert_eq!(cleaned, 5);
    }
}
