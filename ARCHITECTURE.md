# PCCLEAN-APP - Disk Cleaner with DDD & Clean Architecture

![Version](https://img.shields.io/badge/version-2.0.0-blue)
![Architecture](https://img.shields.io/badge/architecture-DDD-green)
![Clean Code](https://img.shields.io/badge/clean%20code-SOLID-orange)

Uma aplicação moderna de limpeza de disco construída com **Domain-Driven Design (DDD)**, **Clean Architecture** e **Clean Code** principles.

## 🏗️ Arquitetura

O projeto segue uma arquitetura em camadas bem definida:

```
src/
├── domain/              # Camada de Domínio (Regras de Negócio)
│   ├── entities/        # Entidades do domínio
│   ├── value_objects/   # Objetos de Valor
│   ├── repositories/    # Interfaces de Repositórios
│   └── services/        # Serviços de Domínio
│
├── application/         # Camada de Aplicação (Casos de Uso)
│   └── use_cases/       # Casos de Uso / Interactors
│
├── infrastructure/      # Camada de Infraestrutura (Implementações)
│   └── repositories/    # Implementações concretas dos repositórios
│
├── presentation/        # Camada de Apresentação (UI)
│   ├── app.rs          # Aplicação principal
│   ├── theme.rs        # Tema e estilos
│   └── widgets/        # Componentes reutilizáveis
│
└── shared/             # Código compartilhado entre camadas
    └── formatters.rs   # Utilitários e helpers
```

## 🎯 Princípios Aplicados

### Domain-Driven Design (DDD)

- **Entities**: `CleanableItem`, `CleanableCategory`, `ScanResult`
- **Value Objects**: `CategoryType`
- **Repositories**: Interfaces que abstraem a persistência/acesso a dados
- **Use Cases**: Lógica de aplicação isolada e testável
- **Aggregates**: `ScanResult` como agregado raiz

### SOLID Principles

✅ **Single Responsibility Principle (SRP)**
- Cada classe tem uma única responsabilidade
- `ScanSystemUseCase` apenas coordena varreduras
- `CleanSelectedCategoriesUseCase` apenas coordena limpezas

✅ **Open/Closed Principle (OCP)**
- Extensível através de novas implementações de repositórios
- Fechado para modificação nas entidades de domínio

✅ **Liskov Substitution Principle (LSP)**
- Implementações de `ScannerRepository` são intercambiáveis
- Implementações de `CleanerRepository` são intercambiáveis

✅ **Interface Segregation Principle (ISP)**
- Interfaces pequenas e específicas
- `ScannerRepository` e `CleanerRepository` separadas

✅ **Dependency Inversion Principle (DIP)**
- Use cases dependem de abstrações (traits)
- Infraestrutura implementa as abstrações
- Injeção de dependências no ponto de entrada

### Clean Code

- **Nomes significativos**: Todos os nomes são claros e descritivos
- **Funções pequenas**: Cada função faz apenas uma coisa
- **Comentários úteis**: Documentação onde necessário
- **Tratamento de erros**: Uso de `Result<T>` do Rust
- **Testes**: Testes unitários nas camadas de domínio e aplicação
- **DRY**: Código reutilizável e sem duplicação

## 🚀 Funcionalidades

- 🔍 **Varredura Inteligente**
  - Arquivos de log
  - Arquivos temporários
  - Imagens Docker não utilizadas
  - Volumes Docker não utilizados
  - Pacotes de desenvolvimento (node_modules, cargo cache)

- 🧹 **Limpeza Segura**
  - Seleção múltipla de categorias
  - Confirmação antes de limpar
  - Feedback em tempo real

- 🎨 **Interface Cyberpunk**
  - Tema neon futurista
  - Animações suaves
  - Componentes reutilizáveis

## 📦 Dependências

```toml
[dependencies]
eframe = { version = "0.28", features = ["wgpu"] }
egui = "0.28"
walkdir = "2.5"
anyhow = "1.0"
```

## 🔧 Como Executar

### Debug
```bash
cargo run
```

### Release (Otimizado)
```bash
cargo run --release
```

### Executar Testes
```bash
cargo test
```

## 📚 Camadas Explicadas

### 1. Domain Layer (Domínio)
A camada mais interna, contendo as regras de negócio puras.

- **Entities**: Objetos com identidade e ciclo de vida
- **Value Objects**: Objetos imutáveis sem identidade
- **Repositories**: Contratos (traits) para acesso a dados
- **Zero dependências** de frameworks ou infraestrutura

### 2. Application Layer (Aplicação)
Orquestra o fluxo de dados entre UI e domínio.

- **Use Cases**: Implementam a lógica de aplicação
- Dependem apenas de abstrações do domínio
- Totalmente testáveis

### 3. Infrastructure Layer (Infraestrutura)
Implementações concretas de acesso a dados e serviços externos.

- **Repositories**: Implementações concretas
- Acessa filesystem, Docker CLI, etc.
- Pode ser substituída sem afetar outras camadas

### 4. Presentation Layer (Apresentação)
Interface com o usuário.

- Usa **egui** para UI
- Componentes reutilizáveis (widgets)
- Tema cyberpunk customizado
- Depende apenas de Use Cases

### 5. Shared Layer (Compartilhado)
Utilitários e helpers usados por múltiplas camadas.

## 🧪 Testabilidade

Cada camada é testável independentemente:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_cleanable_item() {
        let item = CleanableItem::new("/tmp/test.log".to_string(), 1024);
        assert_eq!(item.path(), "/tmp/test.log");
        assert!(item.is_significant());
    }
}
```

## 🎨 Design Patterns

- **Repository Pattern**: Abstração de acesso a dados
- **Use Case Pattern**: Encapsulamento de lógica de aplicação
- **Dependency Injection**: Inversão de controle
- **Strategy Pattern**: Diferentes estratégias de limpeza
- **Presenter Pattern**: Separação entre UI e lógica

## 📝 Próximos Passos

- [ ] Adicionar mais testes de integração
- [ ] Implementar logging estruturado
- [ ] Adicionar métricas de performance
- [ ] Suporte a múltiplos idiomas
- [ ] Histórico de limpezas

## 🤝 Contribuindo

Contribuições são bem-vindas! Por favor, siga os princípios de DDD e Clean Code ao adicionar novas funcionalidades.

## 📄 Licença

MIT License

---

**Desenvolvido com 💜 seguindo DDD, Clean Architecture e Clean Code**
