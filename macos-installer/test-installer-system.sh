#!/bin/bash

# Script de teste do sistema de instalação
# Verifica se todos os componentes estão prontos

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║    PCClean - Teste do Sistema de Instalação         ║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

PASS=0
FAIL=0

check() {
    local result=$?
    if [ $result -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $1"
        PASS=$((PASS + 1))
    else
        echo -e "${RED}✖${NC} $1"
        FAIL=$((FAIL + 1))
    fi
}

# Verificar estrutura de arquivos
echo -e "${CYAN}📁 Verificando estrutura de arquivos...${NC}"
[ -f "${SCRIPT_DIR}/build-installer.sh" ]; check "build-installer.sh existe"
[ -x "${SCRIPT_DIR}/build-installer.sh" ]; check "build-installer.sh é executável"
[ -f "${SCRIPT_DIR}/install.sh" ]; check "install.sh existe"
[ -x "${SCRIPT_DIR}/install.sh" ]; check "install.sh é executável"
[ -f "${SCRIPT_DIR}/uninstall.sh" ]; check "uninstall.sh existe"
[ -x "${SCRIPT_DIR}/uninstall.sh" ]; check "uninstall.sh é executável"
[ -f "${SCRIPT_DIR}/create-icon.sh" ]; check "create-icon.sh existe"
[ -x "${SCRIPT_DIR}/create-icon.sh" ]; check "create-icon.sh é executável"
[ -f "${SCRIPT_DIR}/sign-and-notarize.sh" ]; check "sign-and-notarize.sh existe"
[ -x "${SCRIPT_DIR}/sign-and-notarize.sh" ]; check "sign-and-notarize.sh é executável"
[ -f "${SCRIPT_DIR}/README.md" ]; check "README.md existe"
echo ""

# Verificar documentação
echo -e "${CYAN}📚 Verificando documentação...${NC}"
[ -f "${PROJECT_ROOT}/INSTALL_MACOS.md" ]; check "INSTALL_MACOS.md existe"
[ -f "${PROJECT_ROOT}/RELEASE_GUIDE.md" ]; check "RELEASE_GUIDE.md existe"
[ -f "${PROJECT_ROOT}/README.md" ]; check "README.md principal existe"
echo ""

# Verificar GitHub Actions
echo -e "${CYAN}⚙️  Verificando GitHub Actions...${NC}"
[ -d "${PROJECT_ROOT}/.github/workflows" ]; check ".github/workflows existe"
[ -f "${PROJECT_ROOT}/.github/workflows/build-macos-installer.yml" ]; check "workflow de build existe"
echo ""

# Verificar dependências do sistema
echo -e "${CYAN}🔧 Verificando dependências do sistema...${NC}"
command -v cargo &> /dev/null; check "Rust/Cargo instalado"
command -v git &> /dev/null; check "Git instalado"
command -v iconutil &> /dev/null; check "iconutil disponível"
command -v hdiutil &> /dev/null; check "hdiutil disponível"
command -v security &> /dev/null; check "security disponível"

# Verificar Cargo.toml
if [ -f "${PROJECT_ROOT}/Cargo.toml" ]; then
    VERSION=$(grep '^version' "${PROJECT_ROOT}/Cargo.toml" | head -1 | cut -d'"' -f2)
    [ ! -z "$VERSION" ]; check "Versão encontrada no Cargo.toml: $VERSION"
fi
echo ""

# Verificar se consegue compilar (rápido)
echo -e "${CYAN}🦀 Verificando compilação (quick check)...${NC}"
cd "$PROJECT_ROOT"
if cargo check &> /dev/null; then
    echo -e "${GREEN}✓${NC} Projeto compila sem erros"
    PASS=$((PASS + 1))
else
    echo -e "${RED}✖${NC} Erros de compilação encontrados"
    FAIL=$((FAIL + 1))
fi
echo ""

# Resumo
echo -e "${CYAN}╔═══════════════════════════════════════════════════════╗${NC}"
echo -e "${CYAN}║                     Resumo                           ║${NC}"
echo -e "${CYAN}╚═══════════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "  ${GREEN}Passou:${NC} $PASS"
echo -e "  ${RED}Falhou:${NC} $FAIL"
echo ""

if [ $FAIL -eq 0 ]; then
    echo -e "${GREEN}✓ Sistema de instalação está pronto!${NC}"
    echo ""
    echo -e "${CYAN}Próximos passos:${NC}"
    echo -e "  1. Criar instalador: ${YELLOW}./build-installer.sh${NC}"
    echo -e "  2. Testar instalação: ${YELLOW}./install.sh${NC}"
    echo -e "  3. Criar ícone: ${YELLOW}./create-icon.sh${NC}"
    echo ""
    exit 0
else
    echo -e "${RED}✖ Alguns testes falharam. Verifique os erros acima.${NC}"
    echo ""
    exit 1
fi
