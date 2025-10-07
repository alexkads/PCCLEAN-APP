# 🚀 Guia de Build - PCCLEAN-APP

## ⚡ Comandos Rápidos

### Para Desenvolvimento (RECOMENDADO)
```bash
cargo build              # Build rápido (~2s) - use durante desenvolvimento
cargo run                # Compila e executa rapidamente
```

### Para Testes de Performance
```bash
cargo build --profile fast-release    # Build médio (~35s) - boa performance
cargo run --profile fast-release      # Executa com otimizações médias
```

### Para Release Final
```bash
cargo build --release    # Build lento (~33s) - máxima otimização
cargo run --release      # Executa versão totalmente otimizada
```

---

## 📊 Comparação de Perfis

| Perfil | Comando | Tempo de Build | Tamanho | Performance | Uso |
|--------|---------|----------------|---------|-------------|-----|
| **dev** | `cargo build` | ~2s ⚡ | 35MB | Boa | Desenvolvimento diário |
| **fast-release** | `cargo build --profile fast-release` | ~35s | 6.4MB | Muito boa | Testes de performance |
| **release** | `cargo build --release` | ~33s | 7.0MB | Máxima | Distribuição final |

---

## 🔧 Detalhes dos Perfis

### 1. **dev** (Padrão - Desenvolvimento)
```toml
opt-level = 0        # Sem otimizações
debug = true         # Debug completo
codegen-units = 16   # Paralelização máxima
```
- ✅ Build super rápido (~2s)
- ✅ Ideal para desenvolvimento
- ✅ Debug completo
- ⚠️ Binário grande (35MB)
- ⚠️ Performance moderada

### 2. **fast-release** (Equilíbrio)
```toml
opt-level = 2        # Otimizações balanceadas
lto = "thin"         # LTO rápido
codegen-units = 8    # Alguma paralelização
```
- ✅ Performance muito boa
- ✅ Build razoável (~35s)
- ✅ Binário otimizado (6.4MB)
- ✅ Ideal para testar performance antes do release

### 3. **release** (Máxima Otimização)
```toml
opt-level = 3        # Máxima otimização
lto = true           # LTO completo
codegen-units = 1    # Sem paralelização
strip = true         # Remove símbolos
panic = "abort"      # Menor binário
```
- ✅ Máxima performance
- ✅ Binário pequeno (7.0MB)
- ⚠️ Build mais lento (~33s)
- ⚠️ Use apenas para distribuição final

---

## 🎯 Quando Usar Cada Perfil

### Durante Desenvolvimento 💻
```bash
# Ciclo rápido de desenvolvimento
cargo run
cargo test
cargo check  # Ainda mais rápido, só verifica erros
```

### Antes de Commitar 🔍
```bash
# Testa com otimizações
cargo build --profile fast-release
cargo test --profile fast-release
```

### Para Distribuir 📦
```bash
# Build final otimizado
cargo build --release
```

---

## 💡 Dicas de Performance

### 1. Use `cargo check` para validação rápida
```bash
cargo check  # ~1s - apenas verifica erros, não gera binário
```

### 2. Build incremental (padrão no dev)
O Rust recompila apenas o que mudou - mantenha o workspace limpo.

### 3. Limpar cache quando necessário
```bash
cargo clean              # Remove todos os builds
cargo clean --release    # Remove apenas release
```

### 4. Compilação paralela
O Rust já usa todos os cores disponíveis. Não é necessário configurar.

---

## 🐛 Troubleshooting

### Build muito lento?
```bash
# Limpe o cache e recompile
cargo clean
cargo build
```

### Binário muito grande?
```bash
# Use release (já tem strip = true)
cargo build --release
```

### Precisa de debug no release?
```bash
# Edite Cargo.toml temporariamente
[profile.release]
debug = true  # Adicione esta linha
```

---

## 📈 Benchmarks

Executado em: MacOS (Apple Silicon)

```
┌─────────────────┬──────────────┬──────────┬──────────────┐
│ Perfil          │ Build Time   │ Tamanho  │ Performance  │
├─────────────────┼──────────────┼──────────┼──────────────┤
│ dev             │   ~2s  ⚡⚡⚡  │  35 MB   │   ⭐⭐⭐      │
│ fast-release    │  ~35s  ⚡     │  6.4 MB  │   ⭐⭐⭐⭐⭐   │
│ release         │  ~33s  ⚡     │  7.0 MB  │   ⭐⭐⭐⭐⭐   │
└─────────────────┴──────────────┴──────────┴──────────────┘
```

---

## 🎓 Recomendações

1. **95% do tempo**: Use `cargo run` (dev)
2. **Testar UX**: Use `cargo run --profile fast-release`
3. **Release final**: Use `cargo build --release`

**Lembre-se**: Builds rápidos durante desenvolvimento = mais produtividade! 🚀
