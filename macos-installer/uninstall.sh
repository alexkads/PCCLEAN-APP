#!/bin/bash

# Script de desinstalação do PCClean

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║         PCClean - Desinstalador para macOS           ║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""

# Confirmar desinstalação
echo -e "${YELLOW}⚠️  Isso irá remover o PCClean completamente do seu sistema${NC}"
read -p "Deseja continuar? [y/N] " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${GREEN}Desinstalação cancelada${NC}"
    exit 0
fi

echo ""
echo -e "${GREEN}▶${NC} Removendo PCClean..."

# Localizar e remover aplicativo
REMOVED=0

if [ -d "/Applications/PCClean.app" ]; then
    echo -e "${YELLOW}▶${NC} Removendo de /Applications..."
    rm -rf "/Applications/PCClean.app"
    echo -e "${GREEN}✓${NC} Removido de /Applications"
    REMOVED=1
fi

if [ -d "$HOME/Applications/PCClean.app" ]; then
    echo -e "${YELLOW}▶${NC} Removendo de ~/Applications..."
    rm -rf "$HOME/Applications/PCClean.app"
    echo -e "${GREEN}✓${NC} Removido de ~/Applications"
    REMOVED=1
fi

# Remover dados do usuário (opcional)
echo ""
read -p "Deseja remover também os dados do aplicativo? [y/N] " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    # Remover preferências
    if [ -f "$HOME/Library/Preferences/com.alexkads.pcclean.plist" ]; then
        echo -e "${YELLOW}▶${NC} Removendo preferências..."
        rm -f "$HOME/Library/Preferences/com.alexkads.pcclean.plist"
        echo -e "${GREEN}✓${NC} Preferências removidas"
    fi
    
    # Remover cache
    if [ -d "$HOME/Library/Caches/com.alexkads.pcclean" ]; then
        echo -e "${YELLOW}▶${NC} Removendo cache..."
        rm -rf "$HOME/Library/Caches/com.alexkads.pcclean"
        echo -e "${GREEN}✓${NC} Cache removido"
    fi
    
    # Remover logs
    if [ -d "$HOME/Library/Logs/PCClean" ]; then
        echo -e "${YELLOW}▶${NC} Removendo logs..."
        rm -rf "$HOME/Library/Logs/PCClean"
        echo -e "${GREEN}✓${NC} Logs removidos"
    fi
    
    # Remover Application Support
    if [ -d "$HOME/Library/Application Support/PCClean" ]; then
        echo -e "${YELLOW}▶${NC} Removendo dados de suporte..."
        rm -rf "$HOME/Library/Application Support/PCClean"
        echo -e "${GREEN}✓${NC} Dados de suporte removidos"
    fi
fi

echo ""
if [ $REMOVED -eq 1 ]; then
    echo -e "${GREEN}✓${NC} PCClean foi completamente removido do sistema"
    echo ""
    echo -e "${CYAN}Obrigado por usar o PCClean!${NC}"
    echo -e "Se você tiver sugestões ou encontrou problemas,"
    echo -e "por favor nos avise: https://github.com/alexkads/PCCLEAN-APP/issues"
else
    echo -e "${YELLOW}⚠${NC} PCClean não foi encontrado no sistema"
fi

echo ""
