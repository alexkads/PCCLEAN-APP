```
╔═══════════════════════════════════════════════════════════════════╗
║                                                                   ║
║   ██████╗  ██████╗ ██████╗██╗     ███████╗ █████╗ ███╗   ██╗    ║
║   ██╔══██╗██╔════╝██╔════╝██║     ██╔════╝██╔══██╗████╗  ██║    ║
║   ██████╔╝██║     ██║     ██║     █████╗  ███████║██╔██╗ ██║    ║
║   ██╔═══╝ ██║     ██║     ██║     ██╔══╝  ██╔══██║██║╚██╗██║    ║
║   ██║     ╚██████╗╚██████╗███████╗███████╗██║  ██║██║ ╚████║    ║
║   ╚═╝      ╚═════╝ ╚═════╝╚══════╝╚══════╝╚═╝  ╚═╝╚═╝  ╚═══╝    ║
║                                                                   ║
║              INSTALADOR MACOS - INÍCIO RÁPIDO                    ║
╚═══════════════════════════════════════════════════════════════════╝
```

# 🍎 Instalador macOS - Início Rápido

## ⚡ 3 Comandos para Começar

```bash
cd macos-installer
./test-installer-system.sh  # Testar (30 segundos)
./build-installer.sh        # Criar instalador (1 minuto)
```

## 📦 O Que Você Obtém

- ✅ **PCClean.app** - Bundle macOS nativo
- ✅ **PCClean-2.0.0.dmg** - Instalador drag-and-drop
- ✅ **pcclean.rb** - Fórmula Homebrew

## 🎯 Comandos Disponíveis

| Comando | Função | Tempo |
|---------|--------|-------|
| `./demo.sh` | Ver demonstração | Instantâneo |
| `./test-installer-system.sh` | Testar sistema | 30s |
| `./build-installer.sh` | Criar instalador DMG | 1min |
| `./install.sh` | Instalar localmente | 10s |
| `./create-icon.sh` | Criar ícone | 5s |
| `./sign-and-notarize.sh` | Assinar app | 5min |
| `./uninstall.sh` | Desinstalar | 5s |

## 📚 Documentação

| Arquivo | Para Quem | Conteúdo |
|---------|-----------|----------|
| `README.md` | Desenvolvedores | Guia técnico completo |
| `INSTALL_MACOS.md` | Usuários finais | Como instalar |
| `RELEASE_GUIDE.md` | Mantenedores | Processo de release |
| `INSTALLER_SUMMARY.md` | Técnicos | Resumo detalhado |
| `MACOS_INSTALLER_COMPLETE.md` | Todos | Overview executivo |

## 🚀 Fluxos de Trabalho

### Desenvolvimento (Rápido)
```bash
./install.sh  # Instala em ~/Applications
```

### Distribuição (Completo)
```bash
./build-installer.sh  # Cria DMG + Homebrew
open build/PCClean-2.0.0.dmg
```

### Release (Automático)
```bash
git tag v2.0.0
git push origin v2.0.0
# GitHub Actions faz tudo!
```

## ✅ Status dos Testes

```
✓ Sistema de instalação está pronto!
Passou: 23 testes
Falhou: 0 testes
```

## 📋 Requisitos

- ✅ macOS 10.13+
- ✅ Rust 1.70+
- ✅ Xcode Command Line Tools
- ⚪ ImageMagick (opcional)
- ⚪ Apple Developer Account (opcional)

## 🎨 Estrutura Criada

```
PCClean.app/
├── Contents/
│   ├── Info.plist
│   ├── MacOS/PCClean
│   └── Resources/AppIcon.icns

PCClean-2.0.0.dmg
├── PCClean.app → Applications
└── README.txt

Homebrew/
└── pcclean.rb
```

## 💡 Dicas Rápidas

### Testar Agora
```bash
./test-installer-system.sh && ./demo.sh
```

### Criar Instalador
```bash
./build-installer.sh
ls -lh build/
```

### Instalar Localmente
```bash
./install.sh
open /Applications/PCClean.app
```

### Ver Demonstração
```bash
./demo.sh
```

## 🐛 Problemas?

| Erro | Solução |
|------|---------|
| "Não pode abrir" | `xattr -cr /Applications/PCClean.app` |
| DMG não monta | `hdiutil verify build/*.dmg` |
| Script não executa | `chmod +x *.sh` |

## 📞 Suporte

- 📖 Leia: `README.md` (guia completo)
- 🧪 Execute: `./test-installer-system.sh`
- 💬 Issues: github.com/alexkads/PCCLEAN-APP/issues

## 🎉 Próximos Passos

1. ✅ Testar: `./test-installer-system.sh`
2. ✅ Criar: `./build-installer.sh`
3. ✅ Testar: `./install.sh`
4. 🚀 Distribuir!

---

**Status**: ✅ Pronto para Produção  
**Testes**: 23/23 Passando  
**Documentação**: Completa  

**Desenvolvido com 💜 em 2025**
