# 📦 Instalador macOS - PCClean

Este diretório contém os scripts para criar instaladores profissionais do PCClean para macOS.

## 🎯 O que é criado

1. **Bundle .app** - Aplicativo macOS nativo
2. **DMG Installer** - Arquivo de instalação drag-and-drop
3. **Fórmula Homebrew** - Para distribuição via Homebrew

## 🚀 Uso Rápido

### Criar Instalador Completo

```bash
cd macos-installer
chmod +x build-installer.sh
./build-installer.sh
```

Isso irá:
- ✅ Compilar o binário em modo release
- ✅ Criar o bundle PCClean.app
- ✅ Gerar o arquivo DMG
- ✅ Criar fórmula Homebrew

### Instalação Rápida (sem DMG)

```bash
cd macos-installer
chmod +x install.sh
./install.sh
```

### Criar Ícone Personalizado

```bash
cd macos-installer
chmod +x create-icon.sh
./create-icon.sh
```

**Requisito:** ImageMagick (`brew install imagemagick`)

## 📁 Estrutura Criada

```
macos-installer/
├── build/
│   ├── PCClean.app/              # Bundle da aplicação
│   │   └── Contents/
│   │       ├── MacOS/
│   │       │   └── PCClean       # Binário executável
│   │       ├── Resources/
│   │       │   └── AppIcon.icns  # Ícone do app
│   │       └── Info.plist        # Metadados do app
│   │
│   ├── PCClean-2.0.0.dmg         # Instalador DMG
│   │
│   └── homebrew/
│       └── pcclean.rb            # Fórmula Homebrew
│
├── build-installer.sh            # Script principal
├── create-icon.sh                # Criar ícone
├── install.sh                    # Instalador rápido
└── README.md                     # Este arquivo
```

## 🎨 Personalização

### Alterar Informações do App

Edite `build-installer.sh`:

```bash
APP_NAME="PCClean"
APP_VERSION="2.0.0"
BUNDLE_ID="com.alexkads.pcclean"
```

### Criar Ícone Personalizado

1. Crie uma imagem PNG 1024x1024 pixels
2. Salve como `icon_source.png` neste diretório
3. Execute:

```bash
# Criar iconset
mkdir AppIcon.iconset

# Redimensionar (requer ImageMagick)
sips -z 16 16     icon_source.png --out AppIcon.iconset/icon_16x16.png
sips -z 32 32     icon_source.png --out AppIcon.iconset/icon_16x16@2x.png
sips -z 32 32     icon_source.png --out AppIcon.iconset/icon_32x32.png
sips -z 64 64     icon_source.png --out AppIcon.iconset/icon_32x32@2x.png
sips -z 128 128   icon_source.png --out AppIcon.iconset/icon_128x128.png
sips -z 256 256   icon_source.png --out AppIcon.iconset/icon_128x128@2x.png
sips -z 256 256   icon_source.png --out AppIcon.iconset/icon_256x256.png
sips -z 512 512   icon_source.png --out AppIcon.iconset/icon_256x256@2x.png
sips -z 512 512   icon_source.png --out AppIcon.iconset/icon_512x512.png
sips -z 1024 1024 icon_source.png --out AppIcon.iconset/icon_512x512@2x.png

# Criar .icns
iconutil -c icns AppIcon.iconset -o AppIcon.icns
```

## 📦 Distribuição

### 1. Via DMG (Recomendado)

```bash
# Após build, compartilhe o DMG
open build/
# Compartilhe PCClean-2.0.0.dmg
```

### 2. Via GitHub Releases

```bash
# 1. Criar release no GitHub
gh release create v2.0.0

# 2. Upload do DMG
gh release upload v2.0.0 build/PCClean-2.0.0.dmg
```

### 3. Via Homebrew

```bash
# 1. Criar seu tap
brew tap alexkads/pcclean

# 2. Adicionar fórmula
cp build/homebrew/pcclean.rb /usr/local/Homebrew/Library/Taps/alexkads/homebrew-pcclean/

# 3. Usuários instalam com:
brew install alexkads/pcclean/pcclean
```

## 🔐 Assinatura e Notarização (Opcional)

Para distribuição oficial na App Store ou notarização:

### 1. Obter Certificado de Desenvolvedor

```bash
# Verificar certificados instalados
security find-identity -v -p codesigning
```

### 2. Assinar o App

```bash
codesign --deep --force --verify --verbose \
  --sign "Developer ID Application: Your Name (TEAM_ID)" \
  build/PCClean.app
```

### 3. Notarizar

```bash
# Criar arquivo para notarização
ditto -c -k --keepParent build/PCClean.app build/PCClean.zip

# Enviar para notarização
xcrun notarytool submit build/PCClean.zip \
  --apple-id "your@email.com" \
  --password "app-specific-password" \
  --team-id "TEAM_ID" \
  --wait

# Anexar ticket
xcrun stapler staple build/PCClean.app
```

## 🧪 Testes

### Testar o App Bundle

```bash
# Executar diretamente
./build/PCClean.app/Contents/MacOS/PCClean

# Ou abrir como app
open build/PCClean.app
```

### Testar o DMG

```bash
# Montar DMG
open build/PCClean-2.0.0.dmg

# Testar instalação
# Arraste para Applications
```

### Verificar Assinatura

```bash
codesign -vvv --deep --strict build/PCClean.app
spctl -a -vvv -t install build/PCClean.app
```

## 🐛 Problemas Comuns

### "PCClean não pode ser aberto porque é de um desenvolvedor não identificado"

**Solução 1:** Permitir nas preferências
```
System Preferences > Security & Privacy > Clique em "Open Anyway"
```

**Solução 2:** Remover quarentena
```bash
xattr -cr /Applications/PCClean.app
```

**Solução 3:** Assinar o app (veja seção de assinatura)

### DMG não monta

```bash
# Verificar DMG
hdiutil verify build/PCClean-2.0.0.dmg

# Reparar se necessário
hdiutil attach -mountpoint /tmp/pcclean build/PCClean-2.0.0.dmg
hdiutil detach /tmp/pcclean
```

### App não executa

```bash
# Verificar dependências
otool -L build/PCClean.app/Contents/MacOS/PCClean

# Verificar permissões
ls -l build/PCClean.app/Contents/MacOS/PCClean
chmod +x build/PCClean.app/Contents/MacOS/PCClean
```

## 📊 Requisitos

- macOS 10.13+ (High Sierra ou superior)
- Rust 1.70+
- Xcode Command Line Tools: `xcode-select --install`
- ImageMagick (opcional): `brew install imagemagick`

## 🔄 Atualização

Para criar uma nova versão:

```bash
# 1. Atualizar versão no Cargo.toml
# 2. Atualizar APP_VERSION no build-installer.sh
# 3. Rebuild
./build-installer.sh

# 4. Testar
open build/PCClean-2.0.0.dmg

# 5. Distribuir
# Upload para GitHub Releases ou distribuir via Homebrew
```

## 📚 Referências

- [Apple Bundle Programming Guide](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFBundles/Introduction/Introduction.html)
- [Creating DMG Installers](https://github.com/create-dmg/create-dmg)
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Code Signing Guide](https://developer.apple.com/library/archive/documentation/Security/Conceptual/CodeSigningGuide/Introduction/Introduction.html)

## 📞 Suporte

Problemas? Abra uma issue no GitHub:
https://github.com/alexkads/PCCLEAN-APP/issues

---

**Desenvolvido com 💜 por Alex Kads**
