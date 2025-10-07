#!/bin/bash

# Script de build do instalador para macOS
# PCClean-App - Cyberpunk Disk Cleaner
# Autor: Alex Kads
# Data: 2025

set -e  # Exit on error

# Cores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Configurações
APP_NAME="PCClean"
APP_VERSION="2.0.0"
BUNDLE_ID="com.alexkads.pcclean"
BINARY_NAME="pcclean-app"
DMG_NAME="PCClean-${APP_VERSION}.dmg"

# Diretórios
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="${SCRIPT_DIR}/build"
APP_DIR="${BUILD_DIR}/${APP_NAME}.app"
CONTENTS_DIR="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"

echo -e "${CYAN}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  PCClean-App - Build do Instalador para macOS       ║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""

# Função para log de progresso
log_step() {
    echo -e "${GREEN}▶${NC} $1"
}

log_error() {
    echo -e "${RED}✖${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

log_success() {
    echo -e "${GREEN}✓${NC} $1"
}

# Verificar se estamos no macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    log_error "Este script deve ser executado no macOS"
    exit 1
fi

# Verificar se o Rust está instalado
if ! command -v cargo &> /dev/null; then
    log_error "Rust/Cargo não encontrado. Instale em: https://rustup.rs/"
    exit 1
fi

# Limpar builds anteriores
log_step "Limpando builds anteriores..."
rm -rf "${BUILD_DIR}"
mkdir -p "${BUILD_DIR}"

# Build do binário em modo release
log_step "Compilando binário em modo release..."
cd "${PROJECT_ROOT}"
cargo build --release

if [ ! -f "${PROJECT_ROOT}/target/release/${BINARY_NAME}" ]; then
    log_error "Falha ao compilar o binário"
    exit 1
fi

log_success "Binário compilado com sucesso"

# Criar estrutura do .app bundle
log_step "Criando estrutura do .app bundle..."
mkdir -p "${MACOS_DIR}"
mkdir -p "${RESOURCES_DIR}"

# Copiar binário
log_step "Copiando binário..."
cp "${PROJECT_ROOT}/target/release/${BINARY_NAME}" "${MACOS_DIR}/${APP_NAME}"
chmod +x "${MACOS_DIR}/${APP_NAME}"

# Criar Info.plist
log_step "Criando Info.plist..."
cat > "${CONTENTS_DIR}/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>en</string>
    <key>CFBundleExecutable</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>${BUNDLE_ID}</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>${APP_VERSION}</string>
    <key>CFBundleVersion</key>
    <string>${APP_VERSION}</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2025 Alex Kads. All rights reserved.</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>NSRequiresAquaSystemAppearance</key>
    <false/>
</dict>
</plist>
EOF

# Criar ícone (temporário - texto)
log_step "Criando ícone..."
cat > "${RESOURCES_DIR}/AppIcon.icns.txt" << EOF
# Para adicionar um ícone personalizado:
# 1. Crie um arquivo PNG de 1024x1024 pixels
# 2. Use: iconutil -c icns AppIcon.iconset -o AppIcon.icns
# 3. Coloque o arquivo .icns aqui em Resources/
EOF

# Criar script de launcher (para permitir execução como app)
log_step "Criando launcher..."
cat > "${MACOS_DIR}/${APP_NAME}.sh" << 'EOF'
#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$DIR"
exec "./${APP_NAME}"
EOF
chmod +x "${MACOS_DIR}/${APP_NAME}.sh"

log_success "Bundle .app criado com sucesso"

# Criar DMG
log_step "Criando arquivo DMG..."

# Criar diretório temporário para o DMG
DMG_TMP="${BUILD_DIR}/dmg_tmp"
mkdir -p "${DMG_TMP}"

# Copiar .app para o diretório temporário
cp -R "${APP_DIR}" "${DMG_TMP}/"

# Criar link simbólico para /Applications
ln -s /Applications "${DMG_TMP}/Applications"

# Criar README
cat > "${DMG_TMP}/README.txt" << EOF
PCClean - Cyberpunk Disk Cleaner v${APP_VERSION}
==============================================

INSTALAÇÃO:
-----------
1. Arraste o ícone "PCClean" para a pasta "Applications"
2. Abra o Launchpad e procure por "PCClean"
3. Na primeira execução, você pode precisar:
   - Ir em System Preferences > Security & Privacy
   - Clicar em "Open Anyway" para permitir a execução

FUNCIONALIDADES:
---------------
• Limpeza de arquivos de log
• Remoção de arquivos temporários
• Limpeza de cache do sistema
• Gerenciamento de containers Docker
• Limpeza de pacotes de desenvolvimento

SEGURANÇA:
----------
⚠️  Sempre confirme antes de limpar
⚠️  Faça backup de dados importantes
⚠️  Use com responsabilidade

SUPORTE:
--------
GitHub: https://github.com/alexkads/PCCLEAN-APP
Email: seu-email@example.com

Copyright © 2025 Alex Kads
Licença: MIT
EOF

# Criar o DMG
DMG_PATH="${BUILD_DIR}/${DMG_NAME}"

log_step "Criando arquivo de imagem de disco..."

# Remover DMG anterior se existir
rm -f "${DMG_PATH}"

# Criar DMG temporário
hdiutil create -volname "PCClean ${APP_VERSION}" \
    -srcfolder "${DMG_TMP}" \
    -ov -format UDRW \
    "${BUILD_DIR}/temp.dmg"

# Montar DMG
MOUNT_DIR="/Volumes/PCClean ${APP_VERSION}"
hdiutil attach "${BUILD_DIR}/temp.dmg"

# Aguardar montagem
sleep 2

# Customizar aparência do DMG (opcional)
echo '
   tell application "Finder"
     tell disk "PCClean '"${APP_VERSION}"'"
           open
           set current view of container window to icon view
           set toolbar visible of container window to false
           set statusbar visible of container window to false
           set the bounds of container window to {100, 100, 600, 400}
           set viewOptions to the icon view options of container window
           set arrangement of viewOptions to not arranged
           set icon size of viewOptions to 128
           set position of item "PCClean.app" of container window to {100, 100}
           set position of item "Applications" of container window to {400, 100}
           close
           open
           update without registering applications
           delay 2
     end tell
   end tell
' | osascript || true

# Desmontar
sleep 2
hdiutil detach "${MOUNT_DIR}" || true

# Converter para DMG comprimido final
hdiutil convert "${BUILD_DIR}/temp.dmg" \
    -format UDZO \
    -imagekey zlib-level=9 \
    -o "${DMG_PATH}"

# Limpar arquivos temporários
rm -f "${BUILD_DIR}/temp.dmg"
rm -rf "${DMG_TMP}"

log_success "DMG criado com sucesso"

# Criar instalador via Homebrew (opcional)
log_step "Criando fórmula Homebrew..."
mkdir -p "${BUILD_DIR}/homebrew"

cat > "${BUILD_DIR}/homebrew/pcclean.rb" << EOF
class Pcclean < Formula
  desc "Cyberpunk disk cleaner for macOS"
  homepage "https://github.com/alexkads/PCCLEAN-APP"
  url "https://github.com/alexkads/PCCLEAN-APP/releases/download/v${APP_VERSION}/PCClean-${APP_VERSION}.dmg"
  version "${APP_VERSION}"
  sha256 "REPLACE_WITH_ACTUAL_SHA256"

  def install
    prefix.install "PCClean.app"
    bin.write_exec_script "#{prefix}/PCClean.app/Contents/MacOS/PCClean"
  end

  def caveats
    <<~EOS
      PCClean has been installed.
      
      To run: pcclean
      
      Or open from Applications folder.
    EOS
  end

  test do
    system "#{bin}/pcclean", "--version"
  end
end
EOF

# Calcular SHA256 do DMG
DMG_SHA256=$(shasum -a 256 "${DMG_PATH}" | awk '{print $1}')
sed -i '' "s/REPLACE_WITH_ACTUAL_SHA256/${DMG_SHA256}/" "${BUILD_DIR}/homebrew/pcclean.rb"

log_success "Fórmula Homebrew criada"

# Sumário final
echo ""
echo -e "${CYAN}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║              Build Completo com Sucesso!             ║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}Arquivos gerados:${NC}"
echo -e "  📦 App Bundle:    ${APP_DIR}"
echo -e "  💿 DMG:           ${DMG_PATH}"
echo -e "  🍺 Homebrew:      ${BUILD_DIR}/homebrew/pcclean.rb"
echo ""
echo -e "${GREEN}Tamanho do DMG:${NC} $(du -h "${DMG_PATH}" | cut -f1)"
echo -e "${GREEN}SHA256:${NC} ${DMG_SHA256}"
echo ""
echo -e "${YELLOW}Próximos passos:${NC}"
echo -e "  1. Testar: Montar o DMG e arrastar para Applications"
echo -e "  2. Distribuir: Upload para GitHub Releases"
echo -e "  3. Homebrew: Publicar a fórmula em um tap"
echo ""
echo -e "${CYAN}Para instalar agora:${NC}"
echo -e "  open \"${DMG_PATH}\""
echo ""
