#!/bin/bash

# Script para assinar e notarizar o PCClean para distribuição oficial
# Requer: Apple Developer Account e certificados instalados

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║  PCClean - Assinatura e Notarização para macOS       ║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""

# Configurações
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${SCRIPT_DIR}/build"
APP_BUNDLE="${BUILD_DIR}/PCClean.app"

# Verificar se o app foi buildado
if [ ! -d "$APP_BUNDLE" ]; then
    echo -e "${RED}✖${NC} App bundle não encontrado. Execute ./build-installer.sh primeiro"
    exit 1
fi

# Verificar certificados disponíveis
echo -e "${CYAN}Certificados de assinatura disponíveis:${NC}"
security find-identity -v -p codesigning

echo ""
echo -e "${YELLOW}Configuração de Assinatura:${NC}"
echo ""

# Solicitar informações de assinatura
read -p "Developer ID Application (ex: Developer ID Application: Your Name (TEAM_ID)): " SIGNING_IDENTITY

if [ -z "$SIGNING_IDENTITY" ]; then
    echo -e "${RED}✖${NC} Identidade de assinatura não fornecida"
    exit 1
fi

# Solicitar informações para notarização
read -p "Apple ID (email): " APPLE_ID
read -p "Team ID: " TEAM_ID
read -s -p "App-Specific Password: " APP_PASSWORD
echo ""
echo ""

# Criar entitlements
ENTITLEMENTS="${BUILD_DIR}/entitlements.plist"
cat > "$ENTITLEMENTS" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>com.apple.security.app-sandbox</key>
    <false/>
    <key>com.apple.security.files.user-selected.read-write</key>
    <true/>
    <key>com.apple.security.files.downloads.read-write</key>
    <true/>
    <key>com.apple.security.network.client</key>
    <true/>
</dict>
</plist>
EOF

echo -e "${GREEN}▶${NC} Assinando o aplicativo..."

# Assinar o app
codesign --deep --force --verify --verbose \
    --sign "$SIGNING_IDENTITY" \
    --options runtime \
    --entitlements "$ENTITLEMENTS" \
    "$APP_BUNDLE"

# Verificar assinatura
echo -e "${GREEN}▶${NC} Verificando assinatura..."
codesign --verify --deep --strict --verbose=2 "$APP_BUNDLE"
spctl -a -vvv -t install "$APP_BUNDLE"

echo -e "${GREEN}✓${NC} App assinado com sucesso"

# Criar ZIP para notarização
echo -e "${GREEN}▶${NC} Preparando para notarização..."
NOTARIZE_ZIP="${BUILD_DIR}/PCClean-notarize.zip"
ditto -c -k --keepParent "$APP_BUNDLE" "$NOTARIZE_ZIP"

# Enviar para notarização
echo -e "${GREEN}▶${NC} Enviando para notarização Apple..."
echo -e "${YELLOW}   Isso pode levar alguns minutos...${NC}"

SUBMISSION_ID=$(xcrun notarytool submit "$NOTARIZE_ZIP" \
    --apple-id "$APPLE_ID" \
    --password "$APP_PASSWORD" \
    --team-id "$TEAM_ID" \
    --wait \
    --output-format json | grep -o '"id":"[^"]*"' | cut -d'"' -f4)

if [ -z "$SUBMISSION_ID" ]; then
    echo -e "${RED}✖${NC} Falha ao submeter para notarização"
    exit 1
fi

echo -e "${GREEN}✓${NC} Notarização concluída (ID: $SUBMISSION_ID)"

# Verificar status
echo -e "${GREEN}▶${NC} Verificando status da notarização..."
xcrun notarytool info "$SUBMISSION_ID" \
    --apple-id "$APPLE_ID" \
    --password "$APP_PASSWORD" \
    --team-id "$TEAM_ID"

# Anexar ticket de notarização
echo -e "${GREEN}▶${NC} Anexando ticket de notarização..."
xcrun stapler staple "$APP_BUNDLE"

# Verificar ticket
echo -e "${GREEN}▶${NC} Verificando ticket..."
xcrun stapler validate "$APP_BUNDLE"

echo -e "${GREEN}✓${NC} Ticket anexado com sucesso"

# Recriar DMG com app assinado e notarizado
echo -e "${GREEN}▶${NC} Criando DMG final..."

DMG_TMP="${BUILD_DIR}/dmg_signed_tmp"
mkdir -p "$DMG_TMP"

cp -R "$APP_BUNDLE" "$DMG_TMP/"
ln -s /Applications "$DMG_TMP/Applications"

DMG_FINAL="${BUILD_DIR}/PCClean-2.0.0-signed.dmg"
rm -f "$DMG_FINAL"

hdiutil create -volname "PCClean 2.0.0" \
    -srcfolder "$DMG_TMP" \
    -ov -format UDZO \
    -imagekey zlib-level=9 \
    "$DMG_FINAL"

# Assinar o DMG
echo -e "${GREEN}▶${NC} Assinando DMG..."
codesign --sign "$SIGNING_IDENTITY" "$DMG_FINAL"

# Verificar DMG
echo -e "${GREEN}▶${NC} Verificando DMG..."
codesign --verify --verbose "$DMG_FINAL"

# Limpar arquivos temporários
rm -rf "$DMG_TMP"
rm -f "$NOTARIZE_ZIP"

echo ""
echo -e "${CYAN}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║         Assinatura e Notarização Concluídas!         ║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}Arquivos prontos para distribuição:${NC}"
echo -e "  📦 App assinado:    ${APP_BUNDLE}"
echo -e "  💿 DMG assinado:    ${DMG_FINAL}"
echo ""
echo -e "${GREEN}Verificações finais:${NC}"
spctl -a -t open --context context:primary-signature -v "$APP_BUNDLE"
echo ""
echo -e "${GREEN}O aplicativo está pronto para distribuição!${NC}"
echo -e "Usuários não verão mais avisos de segurança ao instalar."
echo ""
