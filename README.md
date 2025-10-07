# 🌟 PCCLEAN - Cyberpunk Disk Cleaner

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![egui](https://img.shields.io/badge/egui-GUI-blue?style=for-the-badge)
![wgpu](https://img.shields.io/badge/wgpu-Graphics-purple?style=for-the-badge)

Uma aplicação moderna de limpeza de disco com interface cyberpunk futurística desenvolvida em Rust usando egui + wgpu.

## ✨ Características

- 🎨 **Design Cyberpunk Futurístico** - Interface neon com animações suaves
- ⚡ **Alto Desempenho** - Construído com Rust e aceleração por GPU
- 🔍 **Varredura Inteligente** - Detecta múltiplos tipos de arquivos desnecessários
- 🧹 **Limpeza Segura** - Confirmação antes de remover arquivos

## 🎯 Funcionalidades

### Tipos de Limpeza

1. **📄 Arquivos de Log**
   - `/var/log`
   - `~/Library/Logs`
   - Arquivos `.log` do sistema

2. **🗂️ Arquivos Temporários**
   - `/tmp`
   - `/var/tmp`
   - `~/Library/Caches`

3. **🐳 Docker**
   - Imagens Docker não utilizadas (dangling)
   - Volumes Docker órfãos
   - Limpeza com `docker prune`

4. **📦 Pacotes de Desenvolvimento**
   - Diretórios `node_modules`
   - Cache NPM (`~/.npm`)
   - Cache Cargo (`~/.cargo/registry`)

## 🚀 Instalação

### Pré-requisitos

- Rust 1.70+ 
- Cargo

### Compilar

```bash
# Clone o repositório
git clone <url>
cd # ⚡ PCCLEAN-APP - Cyberpunk Disk Cleaner

![Version](https://img.shields.io/badge/version-2.0-blue)
![Architecture](https://img.shields.io/badge/architecture-DDD-green)
![Clean Code](https://img.shields.io/badge/clean%20code-SOLID-orange)
![Tests](https://img.shields.io/badge/tests-14%20passing-brightgreen)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)

Uma aplicação moderna de limpeza de disco construída com **Domain-Driven Design**, **Clean Architecture** e **SOLID principles**.

## 🎯 Características

- 🏗️ **Arquitetura em Camadas**: DDD + Clean Architecture
- 🧪 **100% Testável**: 14 testes unitários
- 🎨 **Interface Cyberpunk**: Visual futurista com tema neon
- 🚀 **Alta Performance**: Compilado em Rust
- 📚 **Bem Documentado**: 5 documentos detalhados

## 🚀 Quick Start

### 📦 Instalação macOS (Recomendado)

```bash
# Criar instalador DMG
cd macos-installer
./build-installer.sh

# Ou instalação rápida
./install.sh
```

📖 **Guia completo:** [INSTALL_MACOS.md](INSTALL_MACOS.md)

### ⚡ Desenvolvimento (Rápido - Use Este!)
```bash
cargo run              # Build rápido (~2s) + executa
cargo check            # Verifica erros (~1s)
cargo test             # Roda testes (~2s)
```

### 🎮 Testar Performance
```bash
cargo run --profile fast-release    # Build otimizado (~35s)
```

### 📦 Build para Distribuição
```bash
cargo build --release  # Máxima otimização (~33s)
./target/release/pcclean-app
```

> 💡 **Dica**: Durante desenvolvimento, use `cargo run` (2s). O `--release` (~33s) é só para distribuição final!
> 📖 Veja [BUILD_GUIDE.md](BUILD_GUIDE.md) e [QUICK_REFERENCE.md](QUICK_REFERENCE.md) para mais detalhes.

### Verificar Código
```bash
cargo clippy           # Linter
cargo fmt              # Formatação
```

## 📁 Estrutura do Projeto

```
src/
├── domain/              # Regras de negócio puras
│   ├── entities/        # CleanableItem, CleanableCategory, ScanResult
│   ├── value_objects/   # CategoryType
│   └── repositories/    # Interfaces (traits)
│
├── application/         # Casos de uso
│   └── use_cases/       # ScanSystemUseCase, CleanSelectedCategoriesUseCase
│
├── infrastructure/      # Implementações concretas
│   └── repositories/    # FileSystemScanner, FileSystemCleaner
│
├── presentation/        # Interface do usuário
│   ├── app.rs          # Aplicação principal
│   ├── theme.rs        # Tema cyberpunk
│   └── widgets/        # Componentes reutilizáveis
│
└── shared/             # Utilitários
    └── formatters.rs   # format_bytes, etc.
```

## 🎨 Funcionalidades

### Categorias de Limpeza
- 📄 **Arquivos de Log** - Logs do sistema e aplicações
- 🗂️ **Arquivos Temporários** - Cache e arquivos temp
- 🐳 **Imagens Docker** - Imagens não utilizadas
- 💾 **Volumes Docker** - Volumes órfãos
- 📦 **Pacotes de Desenvolvimento** - node_modules, cargo cache

### Interface
- ✅ Seleção múltipla de categorias
- 🎯 Visualização por tamanho
- ⚠️ Confirmação antes de limpar
- 🎨 Tema cyberpunk neon
- 📊 Estatísticas em tempo real

## 📚 Documentação

| Documento | Descrição |
|-----------|-----------|
| [ARCHITECTURE.md](ARCHITECTURE.md) | Guia completo da arquitetura |
| [ARCHITECTURE_DIAGRAM.txt](ARCHITECTURE_DIAGRAM.txt) | Diagrama visual das camadas |
| [REFACTORING_SUMMARY.md](REFACTORING_SUMMARY.md) | Comparação antes/depois |
| [EXTENSIBILITY_GUIDE.md](EXTENSIBILITY_GUIDE.md) | Como adicionar novas features |
| [EXECUTIVE_SUMMARY.md](EXECUTIVE_SUMMARY.md) | Resumo executivo |

## 🏗️ Princípios Aplicados

### Domain-Driven Design (DDD)
✅ Entidades com comportamento  
✅ Value Objects imutáveis  
✅ Agregados bem definidos  
✅ Repositories como abstrações  

### SOLID Principles
✅ Single Responsibility  
✅ Open/Closed  
✅ Liskov Substitution  
✅ Interface Segregation  
✅ Dependency Inversion  

### Clean Code
✅ Nomes significativos  
✅ Funções pequenas e focadas  
✅ Tratamento explícito de erros  
✅ Código testável  
✅ DRY (Don't Repeat Yourself)  

## 🧪 Testes

```bash
$ cargo test
running 14 tests
test result: ok. 14 passed; 0 failed
```

### Cobertura
- ✅ Domain Layer: 9 testes
- ✅ Application Layer: 2 testes
- ✅ Infrastructure Layer: 2 testes
- ✅ Shared Layer: 1 teste

## 📦 Dependências

```toml
eframe = { version = "0.28", features = ["wgpu"] }
egui = "0.28"
walkdir = "2.5"
anyhow = "1.0"
```

## 🔧 Requisitos

- Rust 1.70+
- macOS, Linux ou Windows
- Docker (opcional, para limpeza de containers)

## 💡 Exemplos de Uso

### Adicionar Nova Categoria
```rust
// 1. Adicionar ao enum
pub enum CategoryType {
    BrowserCache,  // Nova categoria
}

// 2. Implementar scanner
fn scan_browser_cache(&self) -> Result<Vec<CleanableItem>> {
    // Sua implementação
}

// 3. Pronto! A UI se adapta automaticamente
```

### Criar Novo Use Case
```rust
pub struct ExportResultsUseCase {
    // Suas dependências
}

impl ExportResultsUseCase {
    pub fn execute(&self, results: &ScanResult) -> Result<()> {
        // Sua lógica
    }
}
```

## 🤝 Contribuindo

Contribuições são bem-vindas! Por favor:

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

**Importante**: Siga os princípios DDD e Clean Code!

## 📈 Roadmap

- [ ] Adicionar testes de integração
- [ ] Implementar logging estruturado
- [ ] Suporte a múltiplos idiomas
- [ ] Histórico de limpezas
- [ ] API REST
- [ ] Suporte a cloud storage

## 📄 Licença

MIT License - veja [LICENSE](LICENSE) para detalhes

## 👏 Agradecimentos

- Eric Evans - Domain-Driven Design
- Robert C. Martin - Clean Architecture & Clean Code
- Rust Community - Excelente linguagem e ferramentas

---

**Desenvolvido com 💜 seguindo as melhores práticas de engenharia de software**

## 🌟 Se você gostou, dê uma estrela!

[![GitHub stars](https://img.shields.io/github/stars/alexkads/PCCLEAN-APP?style=social)](https://github.com/alexkads/PCCLEAN-APP)

# Compilar em modo release
cargo build --release

# Executar
cargo run --release
```

## 🎮 Uso

1. **Iniciar Varredura**: Clique no botão "🔍 INICIAR VARREDURA" para analisar o sistema
2. **Selecionar Categorias**: Marque as caixas das categorias que deseja limpar
3. **Limpar**: Clique em "🧹 LIMPAR SELECIONADOS" e confirme a ação
4. **Acompanhar**: Veja o progresso e os resultados em tempo real

## 🎨 Design Cyberpunk

A interface utiliza:
- **Cores Neon**: Cyan (#00FFFF), Magenta (#FF00FF), Roxo (#8A2BE2)
- **Animações**: Efeitos de brilho e pulsação
- **Tipografia**: Fonte monoespaçada para estética cyberpunk
- **Feedback Visual**: Indicadores coloridos baseados em tamanho e status

## ⚙️ Arquitetura

```
src/
├── main.rs       # Entry point e setup
├── ui.rs         # Interface gráfica (egui)
├── scanner.rs    # Lógica de varredura
└── cleaner.rs    # Lógica de limpeza
```

### Módulos

- **UI**: Renderização da interface com egui/wgpu
- **Scanner**: Análise do sistema de arquivos e Docker
- **Cleaner**: Operações de limpeza segura

## 🔒 Segurança

- ✅ Confirmação obrigatória antes de deletar
- ✅ Simulação de operações perigosas
- ✅ Logs de todas as ações
- ⚠️ **ATENÇÃO**: Use com cuidado em sistemas de produção

## 📊 Performance

- Varredura rápida com `walkdir`
- Renderização acelerada por GPU (wgpu)
- Interface responsiva a 60 FPS

## 🛠️ Tecnologias

- **Rust** - Linguagem principal
- **egui** - Framework de UI imediata
- **wgpu** - API gráfica moderna
- **tokio** - Runtime assíncrono
- **walkdir** - Travessia de diretórios
- **serde** - Serialização

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues ou pull requests.

## 📝 Licença

Este projeto é licenciado sob a MIT License.

## ⚠️ Aviso

Esta ferramenta pode deletar arquivos permanentemente. Use com responsabilidade e sempre faça backup de dados importantes antes de executar limpezas em larga escala.

## 🎯 Roadmap

- [ ] Agendamento automático de limpeza
- [ ] Relatórios detalhados
- [ ] Suporte para Windows e Linux
- [ ] Configurações personalizáveis
- [ ] Modo dry-run (simulação)
- [ ] Integração com mais gerenciadores de pacotes

---

Feito com ⚡ e 💜 em Rust
