# 🧹 Limpeza Pós-Migração (egui → Slint)

## 📋 Arquivos Identificados para Remoção

### ❌ Código Legacy do egui

#### 1. **src/presentation/app.rs** (707 linhas)
- **Status**: ⚠️ OBSOLETO
- **Motivo**: Implementação completa do egui (PCCleanApp struct)
- **Substituído por**: `src/presentation/slint_app.rs`
- **Dependências egui**: eframe, egui, ThemeManager (egui)

#### 2. **src/presentation/theme.rs** 
- **Status**: ⚠️ OBSOLETO
- **Motivo**: Tema em egui::Color32
- **Substituído por**: `ui/theme.slint`
- **Importa**: `use eframe::egui;`

#### 3. **src/presentation/themes/** (diretório completo)
- **Status**: ⚠️ OBSOLETO
- **Arquivos**:
  - `themes/mod.rs`
  - `themes/ultra_modern.rs`
- **Motivo**: ThemeManager para egui
- **Substituído por**: `ui/theme.slint` (Theme global)

#### 4. **src/presentation/widgets/** (diretório completo)
- **Status**: ⚠️ OBSOLETO
- **Arquivos**:
  - `widgets/mod.rs`
  - `widgets/category_widget.rs`
- **Motivo**: Widgets customizados do egui
- **Substituído por**: Componentes em `ui/components/`

---

## 📊 Análise de Impacto

### ✅ Arquivos que devem permanecer:
- ✅ `src/presentation/mod.rs` - Apenas exporta `run_app` do Slint
- ✅ `src/presentation/slint_app.rs` - Nova implementação Slint

### ❌ Arquivos para deletar:
- ❌ `src/presentation/app.rs` (707 linhas de egui)
- ❌ `src/presentation/theme.rs` (tema egui)
- ❌ `src/presentation/themes/` (diretório completo)
- ❌ `src/presentation/widgets/` (diretório completo)

---

## 🔍 Verificação de Dependências

### Grep por "egui" no código:
```bash
$ grep -r "egui" src/
src/presentation/app.rs:use eframe::egui;
src/presentation/theme.rs:use eframe::egui;
src/presentation/themes/ultra_modern.rs:use eframe::egui;
src/presentation/widgets/category_widget.rs:use eframe::egui;
```

**Resultado**: Todas as referências estão em arquivos obsoletos! ✅

### Verificação em `Cargo.toml`:
```toml
[dependencies]
slint = "1.8"  # ✅ Novo framework
# eframe, egui = REMOVIDOS ✅
```

**Resultado**: Dependências já limpas! ✅

---

## 📦 Estatísticas de Limpeza

### Antes da Remoção:
```
src/presentation/
├── app.rs                    707 linhas (OBSOLETO)
├── mod.rs                    3 linhas (OK)
├── slint_app.rs              198 linhas (ATUAL)
├── theme.rs                  ~50 linhas (OBSOLETO)
├── themes/
│   ├── mod.rs                ~20 linhas (OBSOLETO)
│   └── ultra_modern.rs       ~150 linhas (OBSOLETO)
└── widgets/
    ├── mod.rs                ~10 linhas (OBSOLETO)
    └── category_widget.rs    ~200 linhas (OBSOLETO)

Total: ~1,338 linhas
```

### Depois da Remoção:
```
src/presentation/
├── mod.rs                    3 linhas
└── slint_app.rs              198 linhas

Total: 201 linhas (-85% de código!)
```

**Economia**: ~1,137 linhas removidas + 2 diretórios

---

## 🚀 Plano de Execução

### Etapa 1: Backup de Segurança
```bash
# Criar branch de backup
git checkout -b backup-before-cleanup
git add .
git commit -m "Backup antes da limpeza pós-migração"
git checkout main
```

### Etapa 2: Remover Arquivos Obsoletos
```bash
# Remover arquivos individuais
rm src/presentation/app.rs
rm src/presentation/theme.rs

# Remover diretórios completos
rm -rf src/presentation/themes/
rm -rf src/presentation/widgets/
```

### Etapa 3: Verificar Compilação
```bash
cargo clean
cargo build
cargo test
cargo run
```

### Etapa 4: Commit das Mudanças
```bash
git add .
git commit -m "🧹 Remove arquivos legacy do egui após migração para Slint

- Remove app.rs (707 linhas) - PCCleanApp obsoleto
- Remove theme.rs - substituído por ui/theme.slint
- Remove diretório themes/ - ThemeManager obsoleto
- Remove diretório widgets/ - substituído por ui/components/

Resultado: -85% de código (-1,137 linhas)
Nova estrutura: apenas mod.rs + slint_app.rs"
```

---

## ⚠️ Verificações Finais

### Checklist de Segurança:
- [ ] Backup criado (branch ou tag)
- [ ] Nenhuma importação de arquivos deletados em código ativo
- [ ] `cargo build` sem erros
- [ ] `cargo test` passando
- [ ] Aplicação roda corretamente
- [ ] UI funciona como esperado

### Testes Funcionais:
- [ ] Botão "Start Scan" funciona
- [ ] Varredura detecta categorias
- [ ] Checkboxes selecionam categorias
- [ ] Botão "Clean Selected" funciona
- [ ] Diálogo de confirmação aparece
- [ ] Limpeza executa e mostra resultado

---

## 🎯 Benefícios da Limpeza

### 1. **Código mais Limpo**
- Apenas código relevante permanece
- Sem confusão entre implementações antigas e novas

### 2. **Build mais Rápido**
- Menos arquivos para compilar
- Menos dependências (egui removido)

### 3. **Manutenção Simplificada**
- Única fonte de verdade (Slint)
- Menos código para entender e manter

### 4. **Tamanho do Binário**
- Potencial redução no tamanho do executável
- Menos código morto incluído

### 5. **Onboarding de Novos Devs**
- Estrutura clara sem código legacy
- Documentação alinhada com implementação atual

---

## 📚 Documentação Relacionada

- ✅ `MIGRACAO_SLINT.md` - Detalhes da migração
- ✅ `MIGRACAO_RESUMO.md` - Resumo executivo
- ✅ `COMPONENTES_SLINT.md` - Nova arquitetura modular
- ✅ `README.md` - Já atualizado com Slint

---

## 🤔 FAQ

**Q: E se precisarmos voltar ao egui?**  
A: O backup está na branch `backup-before-cleanup` e o código egui completo está no histórico do git.

**Q: Os testes vão quebrar?**  
A: Não temos testes de UI específicos do egui. Testes unitários não são afetados.

**Q: Há risco de quebrar a aplicação?**  
A: Risco mínimo. Os arquivos a remover não são referenciados pelo código ativo (apenas por `slint_app.rs` que já está funcional).

---

## ✅ Status de Execução

**Data**: 7 de outubro de 2025  
**Status**: 🟡 PENDENTE APROVAÇÃO  
**Ação Necessária**: Confirmar remoção dos arquivos listados

**Comando para executar:**
```bash
rm src/presentation/app.rs src/presentation/theme.rs
rm -rf src/presentation/themes/ src/presentation/widgets/
cargo build && cargo run
```
