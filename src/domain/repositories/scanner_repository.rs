use crate::domain::entities::ScanResult;
use crate::domain::value_objects::CategoryType;
use anyhow::Result;

/// Interface de repositório para operações de varredura.
/// Define o contrato que as implementações devem seguir (Dependency Inversion Principle).
pub trait ScannerRepository: Send + Sync {
    /// Escaneia o sistema e retorna os resultados.
    fn scan_system(&self) -> Result<ScanResult>;
    
    /// Escaneia uma categoria específica.
    fn scan_category(&self, category_type: CategoryType) -> Result<Vec<crate::domain::entities::CleanableItem>>;
}
