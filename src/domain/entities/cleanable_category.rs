use super::cleanable_item::CleanableItem;

/// Representa uma categoria de itens limpáveis.
/// Agregado raiz que contém múltiplos CleanableItems.
#[derive(Debug, Clone)]
pub struct CleanableCategory {
    name: String,
    items: Vec<CleanableItem>,
}

impl CleanableCategory {
    /// Cria uma nova categoria vazia.
    pub fn new(name: String) -> Self {
        Self {
            name,
            items: Vec::new(),
        }
    }

    /// Adiciona um item à categoria.
    pub fn add_item(&mut self, item: CleanableItem) {
        if item.is_significant() {
            self.items.push(item);
        }
    }

    /// Retorna o nome da categoria.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Retorna referência aos itens.
    pub fn items(&self) -> &[CleanableItem] {
        &self.items
    }

    /// Calcula o tamanho total da categoria.
    pub fn total_size(&self) -> u64 {
        self.items.iter().map(|item| item.size_in_bytes()).sum()
    }

    /// Retorna a quantidade de itens na categoria.
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Verifica se a categoria está vazia.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Retorna os primeiros N itens da categoria.
    #[allow(dead_code)]
    pub fn take(&self, count: usize) -> &[CleanableItem] {
        let end = count.min(self.items.len());
        &self.items[..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_empty_category() {
        let category = CleanableCategory::new("Test".to_string());
        assert_eq!(category.name(), "Test");
        assert!(category.is_empty());
        assert_eq!(category.total_size(), 0);
    }

    #[test]
    fn should_add_items_and_calculate_total() {
        let mut category = CleanableCategory::new("Logs".to_string());
        category.add_item(CleanableItem::new("/tmp/test1.log".to_string(), 1024));
        category.add_item(CleanableItem::new("/tmp/test2.log".to_string(), 2048));
        
        assert_eq!(category.item_count(), 2);
        assert_eq!(category.total_size(), 3072);
    }

    #[test]
    fn should_ignore_insignificant_items() {
        let mut category = CleanableCategory::new("Empty".to_string());
        category.add_item(CleanableItem::new("/tmp/empty.log".to_string(), 0));
        
        assert!(category.is_empty());
    }
}
