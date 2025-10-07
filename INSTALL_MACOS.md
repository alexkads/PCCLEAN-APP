# 🍎 Guia de Instalação para macOS

## 📦 Opções de Instalação

### Opção 1: Instalador DMG (Recomendado)

```bash
# 1. Criar o instalador
cd macos-installer
./build-installer.sh

# 2. Abrir o DMG
open build/PCClean-2.0.0.dmg

# 3. Arrastar PCClean para Applications
```

### Opção 2: Instalação Rápida via Script

```bash
cd macos-installer
./install.sh
```

### Opção 3: Via Homebrew (Em breve)

```bash
brew tap alexkads/pcclean
brew install pcclean
```

### Opção 4: Build Manual

```bash
# Compilar
cargo build --release

# Executar
./target/release/pcclean-app
```

## 🎨 Personalizar Ícone

```bash
cd macos-installer
./create-icon.sh
```

**Requisitos:** `brew install imagemagick`

## 🗑️ Desinstalação

```bash
cd macos-installer
./uninstall.sh
```

## 🔐 Segurança

Na primeira execução, o macOS pode exibir um aviso. Para resolver:

1. **System Preferences** → **Security & Privacy**
2. Clique em **"Open Anyway"**

Ou via terminal:
```bash
xattr -cr /Applications/PCClean.app
```

## 📋 Requisitos do Sistema

- macOS 10.13 (High Sierra) ou superior
- 50 MB de espaço em disco
- Permissões de administrador (opcional)

## 🐛 Solução de Problemas

### App não abre

```bash
# Remover atributos de quarentena
xattr -cr /Applications/PCClean.app

# Verificar permissões
chmod +x /Applications/PCClean.app/Contents/MacOS/PCClean
```

### Erro de dependências

```bash
# Verificar dependências do binário
otool -L /Applications/PCClean.app/Contents/MacOS/PCClean
```

### Reinstalar

```bash
# Desinstalar
cd macos-installer
./uninstall.sh

# Reinstalar
./install.sh
```

## 📞 Suporte

Problemas? Abra uma issue:
https://github.com/alexkads/PCCLEAN-APP/issues

---

Desenvolvido com 💜 por Alex Kads
