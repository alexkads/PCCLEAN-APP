#!/bin/bash

# Script para criar ícone .icns do PCClean
# Requer ImageMagick: brew install imagemagick

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ICONSET_DIR="${SCRIPT_DIR}/AppIcon.iconset"

echo "🎨 Criando ícone para PCClean..."

# Criar diretório iconset
mkdir -p "${ICONSET_DIR}"

# Criar ícone temporário em ASCII art (para teste)
# Em produção, substitua por uma imagem PNG real

cat > "${SCRIPT_DIR}/icon_source.svg" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<svg width="1024" height="1024" viewBox="0 0 1024 1024" xmlns="http://www.w3.org/2000/svg">
  <!-- Fundo escuro cyberpunk -->
  <rect width="1024" height="1024" fill="#0a0a1f"/>
  
  <!-- Gradiente neon -->
  <defs>
    <linearGradient id="neon" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#00ffff;stop-opacity:1" />
      <stop offset="50%" style="stop-color:#ff00ff;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#8a2be2;stop-opacity:1" />
    </linearGradient>
  </defs>
  
  <!-- Círculo principal -->
  <circle cx="512" cy="512" r="400" fill="none" stroke="url(#neon)" stroke-width="20" opacity="0.8"/>
  <circle cx="512" cy="512" r="380" fill="none" stroke="#00ffff" stroke-width="2" opacity="0.5"/>
  
  <!-- Símbolo de limpeza (broom/vassoura) -->
  <g transform="translate(512, 512)">
    <!-- Cabo da vassoura -->
    <rect x="-20" y="-200" width="40" height="300" fill="url(#neon)" rx="10"/>
    
    <!-- Cerdas -->
    <path d="M -60 100 L -60 180 L -40 200 L 40 200 L 60 180 L 60 100 Z" 
          fill="#ff00ff" opacity="0.9"/>
    <rect x="-50" y="120" width="20" height="80" fill="#00ffff" opacity="0.7"/>
    <rect x="-20" y="120" width="20" height="80" fill="#8a2be2" opacity="0.7"/>
    <rect x="10" y="120" width="20" height="80" fill="#00ffff" opacity="0.7"/>
    <rect x="40" y="120" width="20" height="80" fill="#ff00ff" opacity="0.7"/>
  </g>
  
  <!-- Partículas/sparkles -->
  <circle cx="200" cy="200" r="10" fill="#00ffff" opacity="0.6"/>
  <circle cx="824" cy="300" r="8" fill="#ff00ff" opacity="0.6"/>
  <circle cx="300" cy="750" r="12" fill="#8a2be2" opacity="0.6"/>
  <circle cx="750" cy="800" r="10" fill="#00ffff" opacity="0.6"/>
  <circle cx="850" cy="150" r="8" fill="#ff00ff" opacity="0.6"/>
  
  <!-- Texto PC (opcional) -->
  <text x="512" y="900" font-family="monospace" font-size="120" 
        font-weight="bold" fill="url(#neon)" text-anchor="middle">PC</text>
</svg>
EOF

echo "✓ SVG source criado"

# Verificar se ImageMagick está instalado
if command -v convert &> /dev/null; then
    echo "Usando ImageMagick para converter..."
    
    # Criar PNGs em diferentes tamanhos
    convert "${SCRIPT_DIR}/icon_source.svg" -resize 16x16     "${ICONSET_DIR}/icon_16x16.png"
    convert "${SCRIPT_DIR}/icon_source.svg" -resize 32x32     "${ICONSET_DIR}/icon_16x16@2x.png"
    convert "${SCRIPT_DIR}/icon_source.svg" -resize 32x32     "${ICONSET_DIR}/icon_32x32.png"
    convert "${SCRIPT_DIR}/icon_source.svg" -resize 64x64     "${ICONSET_DIR}/icon_32x32@2x.png"
    convert "${SCRIPT_DIR}/icon_source.svg" -resize 128x128   "${ICONSET_DIR}/icon_128x128.png"
    convert "${SCRIPT_DIR}/icon_source.svg" -resize 256x256   "${ICONSET_DIR}/icon_128x128@2x.png"
    convert "${SCRIPT_DIR}/icon_source.svg" -resize 256x256   "${ICONSET_DIR}/icon_256x256.png"
    convert "${SCRIPT_DIR}/icon_source.svg" -resize 512x512   "${ICONSET_DIR}/icon_256x256@2x.png"
    convert "${SCRIPT_DIR}/icon_source.svg" -resize 512x512   "${ICONSET_DIR}/icon_512x512.png"
    convert "${SCRIPT_DIR}/icon_source.svg" -resize 1024x1024 "${ICONSET_DIR}/icon_512x512@2x.png"
    
    echo "✓ PNGs criados"
    
    # Criar .icns
    iconutil -c icns "${ICONSET_DIR}" -o "${SCRIPT_DIR}/AppIcon.icns"
    echo "✓ AppIcon.icns criado"
    
    # Copiar para Resources se já existir
    if [ -d "${SCRIPT_DIR}/build/PCClean.app/Contents/Resources" ]; then
        cp "${SCRIPT_DIR}/AppIcon.icns" "${SCRIPT_DIR}/build/PCClean.app/Contents/Resources/"
        echo "✓ Ícone copiado para o app bundle"
    fi
    
    echo ""
    echo "🎨 Ícone criado com sucesso!"
    echo "Arquivo: ${SCRIPT_DIR}/AppIcon.icns"
    
else
    echo "⚠️  ImageMagick não encontrado. Instale com: brew install imagemagick"
    echo "   Você também pode criar o ícone manualmente:"
    echo "   1. Crie PNGs de 16x16 até 1024x1024"
    echo "   2. Organize em AppIcon.iconset/"
    echo "   3. Execute: iconutil -c icns AppIcon.iconset"
fi

# Limpar
rm -rf "${ICONSET_DIR}"

echo ""
echo "Nota: Para um ícone profissional, crie uma imagem PNG 1024x1024"
echo "      e substitua o icon_source.svg antes de executar este script"
