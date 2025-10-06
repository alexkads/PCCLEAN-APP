use crate::domain::repositories::ScannerRepository;
use crate::domain::entities::ScanResult;
use anyhow::Result;
use std::sync::Arc;

/// Caso de uso: Escanear o sistema em busca de itens limpáveis.
/// Segue o princípio de Single Responsibility - apenas coordena a varredura.
pub struct ScanSystemUseCase {
    scanner_repository: Arc<dyn ScannerRepository>,
}

impl ScanSystemUseCase {
    pub fn new(scanner_repository: Arc<dyn ScannerRepository>) -> Self {
        Self {
            scanner_repository,
        }
    }

    /// Executa a varredura completa do sistema.
    pub fn execute(&self) -> Result<ScanResult> {
        self.scanner_repository.scan_system()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::CleanableCategory;

    struct MockScannerRepository;

    impl ScannerRepository for MockScannerRepository {
        fn scan_system(&self) -> Result<ScanResult> {
            let mut result = ScanResult::new();
            result.add_category(CleanableCategory::new("Test".to_string()));
            Ok(result)
        }

        fn scan_category(&self, _category_type: crate::domain::CategoryType) -> Result<Vec<crate::domain::CleanableItem>> {
            Ok(Vec::new())
        }
    }

    #[test]
    fn should_execute_scan() {
        let repo = Arc::new(MockScannerRepository);
        let use_case = ScanSystemUseCase::new(repo);
        
        let result = use_case.execute();
        assert!(result.is_ok());
    }
}
