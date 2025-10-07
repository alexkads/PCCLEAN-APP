# ⚡ Referência Rápida - PCCLEAN-APP

## 🏃 Comandos do Dia-a-Dia

```bash
# 1. DESENVOLVIMENTO (Use 99% do tempo)
cargo run                    # Build rápido (~2s) + executa

# 2. VERIFICAR ERROS (Mais rápido ainda)
cargo check                  # ~1s - só checa, não compila

# 3. TESTES
cargo test                   # Roda todos os testes

# 4. PREVIEW DE PERFORMANCE
cargo run --profile fast-release    # ~35s - boa performance

# 5. BUILD FINAL (Apenas para distribuição)
cargo build --release        # ~33s - máxima otimização
```

---

## ❓ FAQ - Por que demora?

### P: Por que `cargo build --release` demora 33s?
**R:** Release usa otimizações agressivas:
- ✅ `opt-level = 3` - Máxima otimização do código
- ✅ `lto = true` - Link Time Optimization (mais lento)
- ✅ `codegen-units = 1` - Sem paralelização (melhor otimização)
- ✅ `strip = true` - Remove símbolos de debug

**Isso é NORMAL e DESEJÁVEL para release final!**

### P: Como acelerar durante desenvolvimento?
**R:** Use o perfil dev padrão:
```bash
cargo run              # ~2s (17x mais rápido!)
cargo check            # ~1s (33x mais rápido!)
```

### P: Quando devo usar cada perfil?

| Situação | Comando | Tempo |
|----------|---------|-------|
| 🔨 Desenvolvendo código | `cargo run` | 2s |
| 🔍 Só verificar erros | `cargo check` | 1s |
| 🧪 Rodar testes | `cargo test` | 2s |
| 🎮 Testar performance/UX | `cargo run --profile fast-release` | 35s |
| 📦 Build para distribuir | `cargo build --release` | 33s |

---

## 💡 Dicas Importantes

### ✅ FAÇA
- Use `cargo run` no dia-a-dia
- Use `cargo check` para validação rápida
- Use `--release` apenas quando for distribuir
- Comite com `cargo test` passando

### ❌ NÃO FAÇA
- Não use `--release` durante desenvolvimento
- Não limpe o cache (`cargo clean`) sem necessidade
- Não se preocupe se release demora - é esperado!

---

## 🎯 TL;DR

```bash
# Durante desenvolvimento (USE ESTE! ⚡)
cargo run

# Para distribuir (Use 1x por dia máximo)
cargo build --release
```

**A "demora" do release é uma feature, não um bug!** 
Você ganha um binário 5x menor e muito mais rápido. 🚀
