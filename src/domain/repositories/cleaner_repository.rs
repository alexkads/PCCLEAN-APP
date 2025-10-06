use crate::domain::entities::CleanableCategory;
use anyhow::Result;

/// Interface de repositório para operações de limpeza.
/// Define o contrato que as implementações devem seguir (Dependency Inversion Principle).
pub trait CleanerRepository: Send + Sync {
    /// Limpa os itens de uma categoria específica.
    fn clean_category(&self, category: &CleanableCategory) -> Result<usize>;
    
    /// Verifica se é possível limpar uma categoria.
    fn can_clean(&self, category: &CleanableCategory) -> bool;
}
