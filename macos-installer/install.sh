#!/bin/bash

# Script de instalação rápida do PCClean
# Para uso direto sem DMG

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║      PCClean - Instalador Rápido para macOS          ║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""

# Verificar macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo -e "${RED}✖${NC} Este instalador é apenas para macOS"
    exit 1
fi

# Verificar permissões
if [ ! -w "/Applications" ]; then
    echo -e "${YELLOW}⚠${NC} Requer permissões de administrador para instalar em /Applications"
    echo -e "   Você pode instalar em ~/Applications sem sudo"
    read -p "Instalar em ~/Applications? [Y/n] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Nn]$ ]]; then
        echo -e "${RED}Instalação cancelada${NC}"
        exit 1
    fi
    INSTALL_DIR="$HOME/Applications"
else
    INSTALL_DIR="/Applications"
fi

mkdir -p "$INSTALL_DIR"

# Diretórios
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${SCRIPT_DIR}/build"
APP_BUNDLE="${BUILD_DIR}/PCClean.app"

# Verificar se o app foi buildado
if [ ! -d "$APP_BUNDLE" ]; then
    echo -e "${YELLOW}⚠${NC} App bundle não encontrado. Executando build..."
    "${SCRIPT_DIR}/build-installer.sh"
fi

# Copiar para Applications
echo -e "${GREEN}▶${NC} Instalando PCClean em ${INSTALL_DIR}..."

# Remover versão anterior se existir
if [ -d "${INSTALL_DIR}/PCClean.app" ]; then
    echo -e "${YELLOW}⚠${NC} Removendo versão anterior..."
    rm -rf "${INSTALL_DIR}/PCClean.app"
fi

# Copiar nova versão
cp -R "$APP_BUNDLE" "$INSTALL_DIR/"

# Verificar instalação
if [ -d "${INSTALL_DIR}/PCClean.app" ]; then
    echo -e "${GREEN}✓${NC} PCClean instalado com sucesso!"
    echo ""
    echo -e "${GREEN}Para executar:${NC}"
    echo -e "  • Abra o Launchpad e procure 'PCClean'"
    echo -e "  • Ou execute: open ${INSTALL_DIR}/PCClean.app"
    echo ""
    echo -e "${YELLOW}Primeira execução:${NC}"
    echo -e "  • Vá em System Preferences > Security & Privacy"
    echo -e "  • Clique em 'Open Anyway' se solicitado"
    echo ""
    
    # Perguntar se deseja abrir agora
    read -p "Deseja abrir o PCClean agora? [Y/n] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Nn]$ ]]; then
        open "${INSTALL_DIR}/PCClean.app"
    fi
else
    echo -e "${RED}✖${NC} Falha na instalação"
    exit 1
fi
