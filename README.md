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
cd PCCLEAN-APP

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
