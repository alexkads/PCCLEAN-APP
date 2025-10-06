use super::cleanable_category::CleanableCategory;

/// Representa o resultado completo de uma varredura do sistema.
/// Este é o agregado raiz principal do domínio.
#[derive(Debug, Clone)]
pub struct ScanResult {
    categories: Vec<CleanableCategory>,
}

impl ScanResult {
    /// Cria um novo resultado de varredura vazio.
    pub fn new() -> Self {
        Self {
            categories: Vec::new(),
        }
    }

    /// Adiciona uma categoria ao resultado.
    pub fn add_category(&mut self, category: CleanableCategory) {
        self.categories.push(category);
    }

    /// Retorna todas as categorias.
    pub fn categories(&self) -> &[CleanableCategory] {
        &self.categories
    }

    /// Retorna uma categoria específica por nome.
    pub fn get_category(&self, name: &str) -> Option<&CleanableCategory> {
        self.categories.iter().find(|c| c.name() == name)
    }

    /// Retorna uma categoria mutável por nome.
    pub fn get_category_mut(&mut self, name: &str) -> Option<&mut CleanableCategory> {
        self.categories.iter_mut().find(|c| c.name() == name)
    }

    /// Calcula o tamanho total de todos os itens encontrados.
    pub fn total_size(&self) -> u64 {
        self.categories.iter().map(|cat| cat.total_size()).sum()
    }

    /// Retorna o total de itens em todas as categorias.
    pub fn total_items(&self) -> usize {
        self.categories.iter().map(|cat| cat.item_count()).sum()
    }

    /// Verifica se há algum item encontrado.
    pub fn has_items(&self) -> bool {
        self.categories.iter().any(|cat| !cat.is_empty())
    }
}

impl Default for ScanResult {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::CleanableItem;

    #[test]
    fn should_create_empty_result() {
        let result = ScanResult::new();
        assert_eq!(result.total_size(), 0);
        assert_eq!(result.total_items(), 0);
        assert!(!result.has_items());
    }

    #[test]
    fn should_add_categories_and_calculate_totals() {
        let mut result = ScanResult::new();
        
        let mut cat1 = CleanableCategory::new("Logs".to_string());
        cat1.add_item(CleanableItem::new("/tmp/test.log".to_string(), 1024));
        
        let mut cat2 = CleanableCategory::new("Temp".to_string());
        cat2.add_item(CleanableItem::new("/tmp/temp.txt".to_string(), 2048));
        
        result.add_category(cat1);
        result.add_category(cat2);
        
        assert_eq!(result.total_items(), 2);
        assert_eq!(result.total_size(), 3072);
        assert!(result.has_items());
    }
}
