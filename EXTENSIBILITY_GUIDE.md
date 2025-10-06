# 🚀 Guia de Extensibilidade - Como Adicionar Novas Features

Este guia mostra como a arquitetura DDD facilita a adição de novas funcionalidades.

## 📋 Cenários Práticos

### 1. Adicionar Nova Categoria de Limpeza

#### Passo 1: Adicionar no Value Object
```rust
// src/domain/value_objects/category_type.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CategoryType {
    LogFiles,
    TemporaryFiles,
    DockerImages,
    DockerVolumes,
    DevelopmentPackages,
    BrowserCache,  // ⭐ NOVA CATEGORIA
}

impl CategoryType {
    pub fn display_name(&self) -> &'static str {
        match self {
            // ... casos existentes ...
            CategoryType::BrowserCache => "🌐 Cache de Navegadores",
        }
    }
}
```

#### Passo 2: Implementar Scanner
```rust
// src/infrastructure/repositories/filesystem_scanner_repository.rs

impl FileSystemScannerRepository {
    fn scan_browser_cache(&self) -> Result<Vec<CleanableItem>> {
        let mut items = Vec::new();
        let home = std::env::var("HOME").unwrap_or_default();
        
        // Chrome
        let chrome_cache = format!("{}/Library/Caches/Google/Chrome", home);
        if let Ok(size) = Self::calculate_directory_size(Path::new(&chrome_cache)) {
            if size > 0 {
                items.push(CleanableItem::new(chrome_cache, size));
            }
        }
        
        // Firefox
        let firefox_cache = format!("{}/Library/Caches/Firefox", home);
        if let Ok(size) = Self::calculate_directory_size(Path::new(&firefox_cache)) {
            if size > 0 {
                items.push(CleanableItem::new(firefox_cache, size));
            }
        }
        
        Ok(items)
    }
}

impl ScannerRepository for FileSystemScannerRepository {
    fn scan_category(&self, category_type: CategoryType) -> Result<Vec<CleanableItem>> {
        match category_type {
            // ... casos existentes ...
            CategoryType::BrowserCache => self.scan_browser_cache(),
        }
    }
}
```

#### Passo 3: Atualizar UI (opcional)
A UI já está preparada! Ela renderiza automaticamente todas as categorias.

**Resultado**: Nova categoria funcionando sem quebrar nada! ✅

---

### 2. Adicionar Novo Repositório (Ex: Cloud Storage)

#### Passo 1: Criar Nova Implementação
```rust
// src/infrastructure/repositories/cloud_scanner_repository.rs

use crate::domain::repositories::ScannerRepository;
use crate::domain::entities::{ScanResult, CleanableCategory, CleanableItem};
use crate::domain::value_objects::CategoryType;
use anyhow::Result;

pub struct CloudScannerRepository {
    api_key: String,
}

impl CloudScannerRepository {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
    
    async fn scan_cloud_storage(&self) -> Result<Vec<CleanableItem>> {
        // Implementação para scanear cloud storage
        // Ex: AWS S3, Google Cloud, etc.
        Ok(Vec::new())
    }
}

impl ScannerRepository for CloudScannerRepository {
    fn scan_system(&self) -> Result<ScanResult> {
        // Implementação cloud
        todo!("Implementar scan de cloud")
    }
    
    fn scan_category(&self, category_type: CategoryType) -> Result<Vec<CleanableItem>> {
        // Implementação cloud
        todo!("Implementar scan de categoria na cloud")
    }
}
```

#### Passo 2: Usar no Main
```rust
// src/main.rs

// Escolher qual implementação usar
let scanner_repo = if cfg!(feature = "cloud") {
    Arc::new(CloudScannerRepository::new("api-key".to_string()))
} else {
    Arc::new(FileSystemScannerRepository::new())
};

let scan_use_case = Arc::new(ScanSystemUseCase::new(scanner_repo));
```

**Resultado**: Suporte a cloud sem alterar a lógica existente! ✅

---

### 3. Adicionar Validações no Domínio

#### Exemplo: Validar Tamanho Mínimo
```rust
// src/domain/entities/cleanable_item.rs

impl CleanableItem {
    pub fn new(path: String, size_in_bytes: u64) -> Self {
        Self {
            path,
            size_in_bytes,
        }
    }
    
    /// Retorna true se o item for significativo (> 1MB)
    pub fn is_significant(&self) -> bool {
        self.size_in_bytes > 0
    }
    
    /// ⭐ NOVA VALIDAÇÃO
    pub fn is_large(&self) -> bool {
        const ONE_GB: u64 = 1_073_741_824;
        self.size_in_bytes > ONE_GB
    }
    
    /// ⭐ NOVA VALIDAÇÃO
    pub fn requires_confirmation(&self) -> bool {
        const HUNDRED_MB: u64 = 104_857_600;
        self.size_in_bytes > HUNDRED_MB
    }
}
```

#### Usar na UI
```rust
// src/presentation/widgets/category_widget.rs

fn get_size_color(&self) -> egui::Color32 {
    // Usar a nova lógica do domínio
    if self.category.items().iter().any(|i| i.is_large()) {
        egui::Color32::from_rgb(255, 0, 0) // Vermelho
    } else if self.category.items().iter().any(|i| i.requires_confirmation()) {
        egui::Color32::from_rgb(255, 165, 0) // Laranja
    } else {
        egui::Color32::from_rgb(0, 255, 0) // Verde
    }
}
```

**Resultado**: Lógica de negócio centralizada no domínio! ✅

---

### 4. Adicionar Novo Use Case

#### Exemplo: Export Results to JSON
```rust
// src/application/use_cases/export_results_use_case.rs

use crate::domain::entities::ScanResult;
use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct ExportResultsUseCase;

impl ExportResultsUseCase {
    pub fn new() -> Self {
        Self
    }
    
    pub fn execute(&self, results: &ScanResult, output_path: &Path) -> Result<()> {
        let json = self.serialize_results(results)?;
        let mut file = File::create(output_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
    
    fn serialize_results(&self, results: &ScanResult) -> Result<String> {
        // Serializar para JSON
        let mut output = String::from("{\n");
        
        for category in results.categories() {
            output.push_str(&format!(
                "  \"{}\": {{\n",
                category.name()
            ));
            output.push_str(&format!(
                "    \"total_size\": {},\n",
                category.total_size()
            ));
            output.push_str(&format!(
                "    \"item_count\": {}\n",
                category.item_count()
            ));
            output.push_str("  },\n");
        }
        
        output.push_str("}\n");
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::{CleanableCategory, CleanableItem};
    
    #[test]
    fn should_serialize_results() {
        let use_case = ExportResultsUseCase::new();
        let mut results = ScanResult::new();
        
        let mut category = CleanableCategory::new("Test".to_string());
        category.add_item(CleanableItem::new("/tmp/test.log".to_string(), 1024));
        results.add_category(category);
        
        let json = use_case.serialize_results(&results).unwrap();
        assert!(json.contains("Test"));
        assert!(json.contains("1024"));
    }
}
```

**Resultado**: Novo caso de uso independente e testável! ✅

---

### 5. Adicionar Logging Estruturado

#### Passo 1: Adicionar Dependência
```toml
# Cargo.toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

#### Passo 2: Adicionar no Infrastructure
```rust
// src/infrastructure/repositories/filesystem_scanner_repository.rs

use tracing::{info, warn, error, instrument};

impl FileSystemScannerRepository {
    #[instrument(skip(self))]
    fn scan_log_files(&self) -> Result<Vec<CleanableItem>> {
        info!("Starting log files scan");
        
        let mut items = Vec::new();
        // ... código existente ...
        
        info!(
            items_found = items.len(),
            "Log files scan completed"
        );
        
        Ok(items)
    }
}
```

#### Passo 3: Configurar no Main
```rust
// src/main.rs

fn main() -> eframe::Result<()> {
    // Configurar logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    
    // ... resto do código ...
}
```

**Resultado**: Logging profissional sem poluir o código! ✅

---

### 6. Adicionar Filtros e Ordenação

#### No Domínio
```rust
// src/domain/entities/cleanable_category.rs

impl CleanableCategory {
    /// ⭐ NOVO MÉTODO
    pub fn items_sorted_by_size(&self) -> Vec<&CleanableItem> {
        let mut sorted: Vec<&CleanableItem> = self.items.iter().collect();
        sorted.sort_by(|a, b| b.size_in_bytes().cmp(&a.size_in_bytes()));
        sorted
    }
    
    /// ⭐ NOVO MÉTODO
    pub fn items_larger_than(&self, min_size: u64) -> Vec<&CleanableItem> {
        self.items
            .iter()
            .filter(|item| item.size_in_bytes() > min_size)
            .collect()
    }
    
    /// ⭐ NOVO MÉTODO
    pub fn top_n_items(&self, n: usize) -> Vec<&CleanableItem> {
        self.items_sorted_by_size().into_iter().take(n).collect()
    }
}
```

#### Usar na UI
```rust
// src/presentation/widgets/category_widget.rs

fn render_items(&self, ui: &mut egui::Ui) {
    // Mostrar apenas os TOP 5 maiores itens
    for item in self.category.top_n_items(5) {
        ui.horizontal(|ui| {
            ui.label(item.path());
            ui.label(format_bytes(item.size_in_bytes()));
        });
    }
}
```

**Resultado**: Funcionalidades avançadas mantendo o domínio rico! ✅

---

## 🎯 Padrões de Extensão

### ✅ DO (Faça)
1. **Adicione lógica de negócio no Domínio**
2. **Crie novos Use Cases para orquestração**
3. **Implemente novas Repositories para dados**
4. **Adicione testes para novas funcionalidades**
5. **Mantenha a separação de camadas**

### ❌ DON'T (Não faça)
1. ❌ Não adicione lógica de negócio na UI
2. ❌ Não acesse infraestrutura direto do domínio
3. ❌ Não misture responsabilidades
4. ❌ Não crie acoplamento entre camadas
5. ❌ Não escreva código sem testes

---

## 📚 Checklist para Nova Feature

```
□ 1. Definir entidades/value objects no Domínio (se necessário)
□ 2. Criar/atualizar interfaces de Repository (se necessário)
□ 3. Implementar Use Case na camada de Aplicação
□ 4. Implementar Repository na camada de Infraestrutura
□ 5. Atualizar UI na camada de Apresentação
□ 6. Escrever testes unitários
□ 7. Documentar mudanças
□ 8. Compilar e testar: cargo test
□ 9. Verificar código: cargo clippy
□ 10. Formatar código: cargo fmt
```

---

## 🎓 Conclusão

A arquitetura DDD + Clean Architecture torna o projeto:

- ✅ **Fácil de entender**: Cada camada tem seu propósito
- ✅ **Fácil de estender**: Adicione features sem quebrar o existente
- ✅ **Fácil de testar**: Cada parte é testável isoladamente
- ✅ **Fácil de manter**: Mudanças são localizadas
- ✅ **Fácil de escalar**: Arquitetura suporta crescimento

**A arquitetura não é uma restrição, é uma facilitadora!** 🚀
