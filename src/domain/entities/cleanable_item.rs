/// Representa um item que pode ser limpo do sistema.
/// Esta é uma entidade de domínio pura, sem dependências externas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanableItem {
    path: String,
    size_in_bytes: u64,
}

impl CleanableItem {
    /// Cria um novo item limpável.
    /// 
    /// # Argumentos
    /// * `path` - Caminho completo do item
    /// * `size_in_bytes` - Tamanho em bytes
    pub fn new(path: String, size_in_bytes: u64) -> Self {
        Self {
            path,
            size_in_bytes,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn size_in_bytes(&self) -> u64 {
        self.size_in_bytes
    }

    /// Verifica se o item é significativo (> 0 bytes)
    pub fn is_significant(&self) -> bool {
        self.size_in_bytes > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_cleanable_item() {
        let item = CleanableItem::new("/tmp/test.log".to_string(), 1024);
        assert_eq!(item.path(), "/tmp/test.log");
        assert_eq!(item.size_in_bytes(), 1024);
        assert!(item.is_significant());
    }

    #[test]
    fn should_identify_insignificant_item() {
        let item = CleanableItem::new("/tmp/empty.log".to_string(), 0);
        assert!(!item.is_significant());
    }
}
