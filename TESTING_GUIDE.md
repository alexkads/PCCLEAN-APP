# 🧪 Guia de Teste - Funcionalidade de Limpeza

## Como Testar a Correção

### 📋 Teste 1: Limpeza Básica

1. **Inicie a aplicação**
   ```bash
   cargo run --release
   ```

2. **Execute a varredura**
   - Clique em "🔍 Iniciar Varredura"
   - Aguarde a varredura completar

3. **Selecione categorias**
   - Marque as checkboxes das categorias desejadas
   - Ex: Logs Temporários, Cache npm

4. **Execute a limpeza**
   - Clique em "🧹 Limpar Selecionados"
   - Confirme no diálogo "⚠️ Confirmação"
   - Clique em "✓ Confirmar"

5. **Observe o progresso**
   - Diálogo "🔄 Limpando" aparece
   - Spinner animado durante processamento
   - Status atualiza: "Excluindo arquivos..."

6. **Verifique o resultado**
   - Mensagem final: "✓ Concluído! X itens removidos"
   - Botão "Fechar" aparece
   - Clique para fechar o diálogo

### ✅ Resultado Esperado:
- Arquivos são realmente removidos do sistema
- Contagem precisa de itens removidos
- UI responsiva durante todo o processo
- Diálogo fecha apenas após conclusão

---

### 📋 Teste 2: Verificação de Arquivos Removidos

**Antes da limpeza:**
```bash
# Verificar tamanho de logs temporários
du -sh ~/Library/Logs

# Verificar cache npm
du -sh ~/.npm
```

**Execute a limpeza via aplicação**

**Após a limpeza:**
```bash
# Verificar novamente
du -sh ~/Library/Logs
du -sh ~/.npm

# Deve mostrar redução no tamanho
```

---

### 📋 Teste 3: Múltiplas Categorias

1. Execute varredura
2. Selecione **TODAS** as categorias
3. Execute limpeza
4. Observe contagem total de itens removidos

**Exemplo de resultado esperado:**
```
✓ Concluído! 847 itens removidos
```

---

### 📋 Teste 4: Limpeza Docker (se Docker instalado)

1. Liste imagens e volumes Docker:
   ```bash
   docker images
   docker volume ls
   ```

2. Execute varredura na aplicação

3. Selecione "Imagens Docker" e/ou "Volumes Docker"

4. Execute limpeza

5. Verifique novamente:
   ```bash
   docker images
   docker volume ls
   ```

**Resultado esperado:**
- Imagens/volumes não utilizados removidos
- Mensagem de sucesso na UI

---

### 📋 Teste 5: Comportamento sem Seleção

1. Execute varredura
2. **NÃO** selecione nenhuma categoria
3. Observe botão "🧹 Limpar Selecionados"

**Resultado esperado:**
- Botão deve estar **desabilitado** (cinza)
- Não pode clicar

---

### 📋 Teste 6: Re-scan após Limpeza

1. Execute varredura
2. Limpe algumas categorias
3. Execute varredura novamente

**Resultado esperado:**
- Tamanhos menores
- Menos itens encontrados
- Categorias limpas podem estar vazias

---

## 🐛 Problemas a Reportar

Se encontrar algum dos seguintes problemas, reporte:

❌ **Limpeza não remove arquivos** - arquivos ainda existem após "concluído"  
❌ **Contagem incorreta** - número não bate com arquivos removidos  
❌ **Travamento** - UI congela durante limpeza  
❌ **Erro não exibido** - erro acontece mas UI não mostra  
❌ **Diálogo não fecha** - botão "Fechar" não funciona  

---

## ✅ Checklist de Validação

Marque cada item após testar:

- [ ] Varredura funciona corretamente
- [ ] Seleção de categorias funciona
- [ ] Diálogo de confirmação aparece
- [ ] Limpeza realmente remove arquivos
- [ ] Contagem de itens está correta
- [ ] Progresso é exibido em tempo real
- [ ] Mensagem de conclusão aparece
- [ ] Botão "Fechar" funciona
- [ ] UI continua responsiva
- [ ] Re-scan mostra mudanças

---

## 📊 Logs de Depuração

Para ver logs detalhados durante a limpeza:

```bash
# Execute com output de debug
RUST_LOG=debug cargo run --release

# Ou veja erros no console
cargo run --release 2>&1 | grep -i "erro\|error"
```

---

## 🎯 Casos de Uso Reais

### Caso 1: Limpeza de Logs do Sistema
```
1. Varredura encontrou: 234 arquivos, 1.2 GB
2. Limpeza executada
3. Resultado: ✓ Concluído! 234 itens removidos
4. Espaço liberado: ~1.2 GB
```

### Caso 2: Cache de Desenvolvimento
```
1. npm + yarn + cargo cache
2. Varredura encontrou: 5847 arquivos, 3.8 GB
3. Limpeza executada
4. Resultado: ✓ Concluído! 5847 itens removidos
5. Espaço liberado: ~3.8 GB
```

### Caso 3: Docker Cleanup
```
1. Imagens e volumes não usados
2. Varredura encontrou: 12 imagens, 8 volumes
3. Limpeza executada
4. Resultado: ✓ Concluído! 2 itens removidos
   (Docker prune retorna 1 por imagens + 1 por volumes)
```

---

**Boa Sorte nos Testes! 🚀**
