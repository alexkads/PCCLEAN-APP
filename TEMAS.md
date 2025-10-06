# 🎨 Sistema de Temas - PCCLEAN-APP

## 🚀 Como Usar

Clique no botão **"🎨 Temas"** no canto inferior direito da aplicação para abrir o seletor de temas.

---

## 📚 Temas Disponíveis

### 1. 🛸 HUD Sci-Fi (Padrão)
**Descrição:** Interface HUD espacial com scans circulares e grades técnicas

**Características:**
- 🎯 Grade de fundo animada estilo tech
- 🔵 Cores cyan/azul brilhante (0, 255, 255)
- ⬛ Background escuro espacial
- 📐 Cantos retos (0px radius)
- 🎪 Animações de scan circular
- 💫 Efeitos de brilho cyan

**Inspiração:** Star Trek, Mass Effect, Halo interfaces

---

### 2. 💎 Glassmorphism
**Descrição:** Design moderno com efeito de vidro fosco e blur

**Características:**
- 🪟 Efeito de vidro fosco translúcido
- 🎨 Cores pastéis suaves (azul, roxo, rosa)
- ☀️ Modo claro elegante
- ⭕ Cantos bem arredondados (16px radius)
- 🌫️ Sombras suaves e profundas
- ✨ Espaçamento generoso

**Inspiração:** iOS, macOS Big Sur, Windows 11 Acrylic

---

### 3. 📟 Retro Terminal
**Descrição:** Terminal vintage DOS/Unix com fonte monoespaçada

**Características:**
- 💚 Verde fosforescente clássico (0, 255, 0)
- ⬛ Background preto absoluto
- 🔲 Sem arredondamento (terminal puro)
- 🔤 Fonte monoespaçada obrigatória
- 📺 Estilo CRT/terminal antigo
- ⚡ Sem sombras (flat design)

**Inspiração:** DOS, UNIX terminals, Matrix, Hacker aesthetic

**Alternativa:** Disponível em âmbar (255, 180, 0) para terminais Unix clássicos

---

### 4. 🌸 Minimal Zen
**Descrição:** Minimalismo japonês com muito espaço em branco

**Características:**
- 🤍 Background papel branco (252, 252, 253)
- 🖤 Texto tinta preta suave (30, 30, 35)
- 🌸 Acentos sutis: sakura pink, bamboo green, sky blue
- 🎌 Filosofia "Ma" (間) - espaço negativo
- 🕊️ Sombras quase imperceptíveis
- 🧘 Bordas minimalistas

**Inspiração:** Muji, Wabi-sabi, Design japonês, iOS minimalista

---

### 5. ⚡ Neon Cyberpunk
**Descrição:** Neon vibrante estilo Blade Runner com brilho intenso

**Características:**
- 💗 Neon rosa vibrante (255, 0, 150)
- 🔵 Neon cyan brilhante (0, 255, 255)
- 💜 Neon roxo intenso (180, 0, 255)
- 🌃 Background escuro urbano noturno
- ✨ Efeitos de glow/brilho intenso
- 🎆 Bordas neon vibrantes (3-3.5px)

**Inspiração:** Blade Runner, Cyberpunk 2077, Ghost in the Shell, Akira

---

### 6. 🌿 Nature Organic
**Descrição:** Cores naturais inspiradas em floresta e terra

**Características:**
- 🌲 Verde floresta (60, 100, 70)
- 🟤 Marrom terra (130, 90, 70)
- 🟠 Laranja argila (200, 130, 90)
- 🌾 Background creme/linho (245, 240, 230)
- ☁️ Sombras suaves e naturais
- 🌱 Cantos orgânicos (8px radius)

**Inspiração:** Material Design Earth tones, Scandinavian design, Natural UI

---

## 🛠️ Arquitetura do Sistema

```
src/presentation/themes/
├── mod.rs                  # Módulo principal
├── theme_manager.rs        # Gerenciador de temas
├── hud_scifi.rs           # Tema HUD Sci-Fi
├── glassmorphism.rs       # Tema Glassmorphism
├── retro_terminal.rs      # Tema Retro Terminal
├── minimal_zen.rs         # Tema Minimal Zen
├── neon_cyberpunk.rs      # Tema Neon Cyberpunk
└── nature_organic.rs      # Tema Nature Organic
```

### ThemeType Enum
```rust
pub enum ThemeType {
    HudSciFi,
    Glassmorphism,
    RetroTerminal,
    MinimalZen,
    NeonCyberpunk,
    NatureOrganic,
}
```

### ThemeManager
- Gerencia o tema atual
- Controla o seletor de temas
- Aplica temas dinamicamente
- Permite troca em tempo real

---

## 🎨 Paletas de Cores

### HUD Sci-Fi
```
Cyan Primary:    rgb(0, 255, 255)
Cyan Dark:       rgb(0, 180, 200)
BG Dark:         rgb(5, 15, 25)
```

### Glassmorphism
```
Accent Blue:     rgb(120, 140, 255)
Accent Purple:   rgb(200, 120, 255)
BG Glass:        rgba(240, 245, 255, 200)
```

### Retro Terminal
```
Green Bright:    rgb(0, 255, 0)
Green Medium:    rgb(0, 200, 0)
BG Black:        rgb(0, 0, 0)
```

### Minimal Zen
```
Ink Black:       rgb(30, 30, 35)
Sakura Pink:     rgb(255, 200, 210)
Paper White:     rgb(252, 252, 253)
```

### Neon Cyberpunk
```
Neon Pink:       rgb(255, 0, 150)
Neon Cyan:       rgb(0, 255, 255)
BG Void:         rgb(5, 0, 10)
```

### Nature Organic
```
Forest Green:    rgb(60, 100, 70)
Earth Brown:     rgb(130, 90, 70)
BG Cream:        rgb(245, 240, 230)
```

---

## 🚀 Implementação

Cada tema implementa a função `apply()` que configura:
- ✅ Cores de fundo
- ✅ Cores de texto
- ✅ Estilos de widgets
- ✅ Bordas e strokes
- ✅ Arredondamento de cantos
- ✅ Sombras
- ✅ Espaçamento
- ✅ Margens

### Exemplo de Uso
```rust
// No app.rs
self.theme_manager.current_theme().apply(ctx);
```

---

## 💡 Dicas de UX

- **HUD Sci-Fi**: Ideal para quem gosta de tech/gaming
- **Glassmorphism**: Perfeito para usuários macOS/iOS
- **Retro Terminal**: Para nostálgicos e hackers
- **Minimal Zen**: Quando precisa de foco e concentração
- **Neon Cyberpunk**: Para mood noturno e estética futurista
- **Nature Organic**: Ambiente relaxante e acolhedor

---

## 🔮 Futuro

Possíveis expansões:
- [ ] Persistência de tema (salvar preferência)
- [ ] Temas customizáveis pelo usuário
- [ ] Importar/exportar temas
- [ ] Tema "Auto" (segue sistema operacional)
- [ ] Transições animadas entre temas
- [ ] Mais variações (Material, Fluent, etc.)

---

## 📝 Notas Técnicas

- Todos os temas usam `egui::Style` e `egui::Visuals`
- Aplicação em tempo real sem restart
- Zero overhead de performance
- Compatível com todas as features existentes
- Mantém animações específicas por tema

---

**Desenvolvido com ❤️ usando Rust + egui**
