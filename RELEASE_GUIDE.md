# 🚀 Guia de Release - PCClean macOS

## 📋 Checklist de Release

Antes de criar um release:

- [ ] Todos os testes passando (`cargo test`)
- [ ] Código formatado (`cargo fmt`)
- [ ] Sem warnings do clippy (`cargo clippy`)
- [ ] Versão atualizada no `Cargo.toml`
- [ ] CHANGELOG atualizado
- [ ] README atualizado
- [ ] Documentação revisada

## 🔄 Processo de Release

### 1. Atualizar Versão

```bash
# Editar Cargo.toml
# Mudar: version = "2.1.0"

# Editar macos-installer/build-installer.sh
# Mudar: APP_VERSION="2.1.0"

# Commit
git add Cargo.toml macos-installer/build-installer.sh
git commit -m "chore: bump version to 2.1.0"
```

### 2. Criar Tag

```bash
# Criar tag
git tag -a v2.1.0 -m "Release v2.1.0"

# Push tag
git push origin v2.1.0
```

### 3. Build Local (Opcional)

```bash
# Build e teste local
cd macos-installer
./build-installer.sh

# Testar DMG
open build/PCClean-2.1.0.dmg
```

### 4. GitHub Actions (Automático)

O push da tag irá:
1. ✅ Compilar o projeto
2. ✅ Executar testes
3. ✅ Criar instalador DMG
4. ✅ Fazer upload para GitHub Releases
5. ✅ Gerar checksums
6. ✅ Incluir fórmula Homebrew

### 5. Verificar Release

```bash
# Via GitHub CLI
gh release view v2.1.0

# Baixar assets
gh release download v2.1.0
```

## 🔐 Release Assinado (Desenvolvedor Apple)

Para releases oficiais com assinatura:

### Pré-requisitos

1. **Apple Developer Account** ($99/ano)
2. **Certificado Developer ID** instalado
3. **App-Specific Password** gerado

### Configurar Secrets no GitHub

```bash
# Settings > Secrets > Actions > New repository secret

APPLE_ID: seu-email@icloud.com
APPLE_TEAM_ID: ABCD123456
APPLE_APP_PASSWORD: xxxx-xxxx-xxxx-xxxx
APPLE_SIGNING_IDENTITY: "Developer ID Application: Your Name (TEAM_ID)"
```

### Build Assinado

```bash
# Local
cd macos-installer
./sign-and-notarize.sh

# Seguir prompts interativos
```

### Workflow Assinado

Criar `.github/workflows/build-signed-release.yml`:

```yaml
- name: Import Code Signing Certificate
  run: |
    echo "${{ secrets.APPLE_CERTIFICATE }}" | base64 --decode > certificate.p12
    security create-keychain -p "${{ secrets.KEYCHAIN_PASSWORD }}" build.keychain
    security import certificate.p12 -k build.keychain -P "${{ secrets.CERTIFICATE_PASSWORD }}"
    
- name: Sign and Notarize
  env:
    APPLE_ID: ${{ secrets.APPLE_ID }}
    APPLE_TEAM_ID: ${{ secrets.APPLE_TEAM_ID }}
    APPLE_PASSWORD: ${{ secrets.APPLE_APP_PASSWORD }}
  run: ./macos-installer/sign-and-notarize.sh
```

## 📦 Distribuição

### GitHub Releases

```bash
# Manual via gh CLI
gh release create v2.1.0 \
  macos-installer/build/PCClean-2.1.0.dmg \
  --title "PCClean v2.1.0" \
  --notes "Release notes here"

# Ou via web
# https://github.com/alexkads/PCCLEAN-APP/releases/new
```

### Homebrew Tap

```bash
# 1. Criar tap (primeira vez)
brew tap-new alexkads/pcclean

# 2. Copiar fórmula
cp macos-installer/build/homebrew/pcclean.rb \
   $(brew --repository)/Library/Taps/alexkads/homebrew-pcclean/

# 3. Commit e push
cd $(brew --repository)/Library/Taps/alexkads/homebrew-pcclean
git add pcclean.rb
git commit -m "Update to v2.1.0"
git push

# 4. Usuários instalam com:
brew install alexkads/pcclean/pcclean
```

### Cask (App Bundle)

Criar `Casks/pcclean.rb`:

```ruby
cask "pcclean" do
  version "2.1.0"
  sha256 "abc123..."

  url "https://github.com/alexkads/PCCLEAN-APP/releases/download/v#{version}/PCClean-#{version}.dmg"
  name "PCClean"
  desc "Cyberpunk disk cleaner"
  homepage "https://github.com/alexkads/PCCLEAN-APP"

  app "PCClean.app"

  zap trash: [
    "~/Library/Preferences/com.alexkads.pcclean.plist",
    "~/Library/Caches/com.alexkads.pcclean",
  ]
end
```

## 🔍 Verificação de Release

### Checklist Pós-Release

- [ ] DMG disponível no GitHub Releases
- [ ] Checksums corretos
- [ ] DMG instala corretamente
- [ ] App executa sem erros
- [ ] Sem avisos de segurança (se assinado)
- [ ] Fórmula Homebrew funciona
- [ ] README atualizado com nova versão
- [ ] Release notes publicadas

### Testes

```bash
# Download e verificar checksum
curl -LO https://github.com/alexkads/PCCLEAN-APP/releases/download/v2.1.0/PCClean-2.1.0.dmg
curl -LO https://github.com/alexkads/PCCLEAN-APP/releases/download/v2.1.0/checksums.txt
shasum -c checksums.txt

# Testar instalação
open PCClean-2.1.0.dmg
# Arrastar para Applications
# Executar app

# Testar Homebrew
brew uninstall pcclean || true
brew install alexkads/pcclean/pcclean
pcclean --version
```

## 📊 Métricas

Acompanhar após release:

- **Downloads**: GitHub Insights > Traffic
- **Instalações Homebrew**: `brew info pcclean --analytics`
- **Issues**: GitHub Issues
- **Feedback**: Discussões

## 🐛 Hotfix

Para correções urgentes:

```bash
# 1. Branch hotfix
git checkout -b hotfix/2.1.1

# 2. Fazer correção
# ... editar código ...

# 3. Commit
git commit -m "fix: critical bug"

# 4. Merge para main
git checkout main
git merge hotfix/2.1.1

# 5. Tag e push
git tag -a v2.1.1 -m "Hotfix v2.1.1"
git push origin main v2.1.1

# 6. GitHub Actions faz o resto
```

## 📝 Release Notes Template

```markdown
## 🎉 PCClean v2.1.0

### ✨ New Features
- Feature 1 description
- Feature 2 description

### 🐛 Bug Fixes
- Fix 1 description
- Fix 2 description

### 🔧 Improvements
- Improvement 1
- Improvement 2

### 📦 Installation

**DMG Installer:**
1. Download `PCClean-2.1.0.dmg`
2. Open and drag to Applications
3. Enjoy!

**Homebrew:**
\`\`\`bash
brew install alexkads/pcclean/pcclean
\`\`\`

**Manual Build:**
\`\`\`bash
cargo build --release
\`\`\`

### 📊 Checksums
See `checksums.txt` for SHA256 hashes

### 🙏 Contributors
Thanks to all contributors!

**Full Changelog**: https://github.com/alexkads/PCCLEAN-APP/compare/v2.0.0...v2.1.0
```

## 🔄 Ciclo de Release

### Semantic Versioning

- **MAJOR** (3.0.0): Mudanças breaking
- **MINOR** (2.1.0): Novas features compatíveis
- **PATCH** (2.0.1): Bug fixes

### Cadência Sugerida

- **Patch**: Conforme necessário (bugs críticos)
- **Minor**: Mensal (novas features)
- **Major**: Anual (mudanças significativas)

## 📞 Suporte

Problemas no processo de release?
1. Checar GitHub Actions logs
2. Revisar este guia
3. Abrir issue: https://github.com/alexkads/PCCLEAN-APP/issues

---

**Última atualização:** 2025
