# 🎨 Arquitetura de Componentes Slint (Estilo ReactJS)

## 📋 Visão Geral

A interface foi refatorada para seguir o padrão modular do **ReactJS**, onde cada componente vive em seu próprio arquivo. Isso melhora a manutenibilidade, reutilização e organização do código.

## 📁 Estrutura de Diretórios

```
ui/
├── app.slint                 # Aplicação principal (AppWindow)
├── theme.slint              # Tema global (cores, estilos)
├── types.slint              # Tipos e structs compartilhados
└── components/              # Componentes modulares
    ├── ActionButtons.slint  # Botões de ação (Scan/Clean)
    ├── CategoryCard.slint   # Card de categoria individual
    ├── CategoriesList.slint # Lista de categorias
    ├── ConfirmDialog.slint  # Modal de confirmação
    ├── GlassCard.slint      # Card base com glassmorphism
    ├── Header.slint         # Cabeçalho da aplicação
    ├── ModernButton.slint   # Botão customizado
    ├── StatsCard.slint      # Card de estatísticas
    └── StatusBar.slint      # Barra de status
```

## 🧩 Componentes

### 1. **Theme** (`theme.slint`)
```slint
export global Theme {
    out property <color> background: #0a0a14;
    out property <color> primary: #8b5cf6;
    // ... outras cores
}
```
**Responsabilidade**: Tema global acessível por todos os componentes.

---

### 2. **Types** (`types.slint`)
```slint
export struct CategoryData {
    name: string,
    items-count: int,
    total-size: string,
    selected: bool,
    icon: string,
}
```
**Responsabilidade**: Definições de tipos compartilhados.

---

### 3. **GlassCard** (`components/GlassCard.slint`)
```slint
export component GlassCard inherits Rectangle {
    in property <string> title: "";
    in property <bool> glow: false;
    
    // Glassmorphism effect
    background: rgba(20, 20, 40, 0.6);
    border-radius: 20px;
    // ...
    
    @children
}
```
**Responsabilidade**: Card base reutilizável com efeito glassmorphism.  
**Props**: `title`, `glow`  
**Uso**: Container para outros componentes

---

### 4. **ModernButton** (`components/ModernButton.slint`)
```slint
export component ModernButton inherits Rectangle {
    in property <string> text: "";
    in property <string> icon: "";
    in property <bool> primary: true;
    in property <bool> enabled: true;
    callback clicked;
}
```
**Responsabilidade**: Botão customizado com hover effects.  
**Props**: `text`, `icon`, `primary`, `enabled`  
**Callbacks**: `clicked()`  
**Estados**: hover, pressed, disabled

---

### 5. **CategoryCard** (`components/CategoryCard.slint`)
```slint
export component CategoryCard inherits Rectangle {
    in property <CategoryData> data;
    in-out property <bool> selected: data.selected;
    callback selection-changed(bool);
}
```
**Responsabilidade**: Card individual de categoria com checkbox.  
**Props**: `data` (CategoryData), `selected`  
**Callbacks**: `selection-changed(checked)`  
**Features**: Hover animation, glassmorphism

---

### 6. **StatusBar** (`components/StatusBar.slint`)
```slint
export component StatusBar inherits Rectangle {
    in property <string> status: "READY";
    in property <bool> is-busy: false;
    in property <string> total-size: "0 GB";
    in property <int> total-items: 0;
}
```
**Responsabilidade**: Barra de status com indicadores em tempo real.  
**Props**: `status`, `is-busy`, `total-size`, `total-items`  
**Features**: Pulse animation quando busy

---

### 7. **ConfirmDialog** (`components/ConfirmDialog.slint`)
```slint
export component ConfirmDialog inherits Rectangle {
    in property <bool> show: false;
    callback confirmed;
    callback cancelled;
}
```
**Responsabilidade**: Modal de confirmação centralizado.  
**Props**: `show`  
**Callbacks**: `confirmed()`, `cancelled()`  
**Features**: Overlay escuro, animação de entrada

---

### 8. **Header** (`components/Header.slint`)
```slint
export component Header inherits GlassCard {
    // Logo e título
}
```
**Responsabilidade**: Cabeçalho com logo e título.  
**Herda de**: `GlassCard`  
**Features**: Glow effect sempre ativo

---

### 9. **StatsCard** (`components/StatsCard.slint`)
```slint
export component StatsCard inherits GlassCard {
    in property <bool> has-results: false;
    in property <bool> is-scanning: false;
    in property <string> total-size-display: "0.00 GB";
    in property <int> total-items: 0;
}
```
**Responsabilidade**: Card de estatísticas com 3 estados.  
**Props**: `has-results`, `is-scanning`, `total-size-display`, `total-items`  
**Estados**:
- Welcome (antes do scan)
- Scanning (com spinner)
- Results (mostra tamanho total)

---

### 10. **ActionButtons** (`components/ActionButtons.slint`)
```slint
export component ActionButtons inherits HorizontalLayout {
    in property <bool> is-scanning: false;
    in property <bool> is-cleaning: false;
    in property <bool> has-results: false;
    in property <int> categories-count: 0;
    
    callback start-scan;
    callback clean-clicked;
}
```
**Responsabilidade**: Grupo de botões de ação.  
**Callbacks**: `start-scan()`, `clean-clicked()`  
**Features**: Lógica de enable/disable integrada

---

### 11. **CategoriesList** (`components/CategoriesList.slint`)
```slint
export component CategoriesList inherits VerticalLayout {
    in property <[CategoryData]> categories: [];
    callback category-selection-changed(int, bool);
}
```
**Responsabilidade**: Lista renderizada de categorias.  
**Props**: `categories` (array)  
**Callbacks**: `category-selection-changed(index, checked)`  
**Features**: Loop automático com `for`

---

### 12. **AppWindow** (`app.slint`)
```slint
export component AppWindow inherits Window {
    // Imports
    import { StatusBar } from "components/StatusBar.slint";
    import { Header } from "components/Header.slint";
    // ...
    
    // Composição
    VerticalLayout {
        StatusBar { }
        ScrollView {
            Header { }
            StatsCard { }
            ActionButtons { }
            CategoriesList { }
        }
    }
    
    ConfirmDialog { }
}
```
**Responsabilidade**: Orquestração dos componentes.  
**Props**: Estado global (`is-scanning`, `categories`, etc.)  
**Callbacks**: Lógica principal da aplicação

---

## 🔄 Fluxo de Dados

```
AppWindow (State)
    ↓ Props
StatusBar ← status, is-busy, total-size, total-items
    ↓
Header (Static)
    ↓
StatsCard ← has-results, is-scanning, total-size-display
    ↓
ActionButtons ← is-scanning, has-results
    ↓ Callback
    start-scan() → AppWindow.start-scan()
    ↓
CategoriesList ← categories[]
    ↓
CategoryCard (Loop) ← data
    ↓ Callback
    selection-changed(checked) → AppWindow.category-selection-changed(index, checked)
    ↓
ConfirmDialog ← show
    ↓ Callbacks
    confirmed() → AppWindow.confirm-clean()
    cancelled() → AppWindow.cancel-clean()
```

---

## 🎯 Benefícios da Arquitetura Modular

### ✅ **Manutenibilidade**
- Cada componente tem responsabilidade única
- Fácil localizar e corrigir bugs
- Mudanças isoladas não afetam outros componentes

### ✅ **Reutilização**
- `GlassCard` é usado por `Header` e `StatsCard`
- `ModernButton` é usado por múltiplos componentes
- Componentes podem ser exportados para outros projetos

### ✅ **Testabilidade**
- Componentes podem ser testados isoladamente
- Props e callbacks claramente definidos
- Estado interno encapsulado

### ✅ **Escalabilidade**
- Adicionar novos componentes sem modificar existentes
- Hierarquia clara de dependências
- Imports explícitos facilitam refatoração

### ✅ **Colaboração**
- Múltiplos desenvolvedores podem trabalhar em componentes diferentes
- Conflitos de merge minimizados
- Código autodocumentado

---

## 📦 Como Adicionar um Novo Componente

### 1. Criar arquivo em `ui/components/`
```bash
touch ui/components/MyComponent.slint
```

### 2. Definir o componente
```slint
import { Theme } from "../theme.slint";

export component MyComponent inherits Rectangle {
    in property <string> my-prop: "";
    callback my-callback();
    
    // Implementation
    background: Theme.surface;
}
```

### 3. Importar em `app.slint`
```slint
import { MyComponent } from "components/MyComponent.slint";
```

### 4. Usar no layout
```slint
MyComponent {
    my-prop: "Hello";
    my-callback => {
        // Handle event
    }
}
```

---

## 🔧 Convenções de Código

### **Nomes de Arquivos**
- PascalCase: `ModernButton.slint`
- Um componente por arquivo
- Nome do arquivo = nome do componente

### **Propriedades**
- kebab-case: `my-property`
- `in`: propriedade de entrada (readonly)
- `out`: propriedade de saída
- `in-out`: propriedade bidirecional

### **Callbacks**
- kebab-case: `my-callback`
- Definidos com `callback nome(params);`
- Implementados com `nome => { }`

### **Imports**
- Caminhos relativos: `"../theme.slint"`
- Imports de componentes primeiro, depois stdlib

### **Comentários**
- Documentar responsabilidade no topo
- Seções comentadas com `// Section name`

---

## 🚀 Próximos Passos

### 1. **Adicionar Animações**
```slint
animate background {
    duration: 300ms;
    easing: ease-in-out;
}
```

### 2. **Estados Complexos**
```slint
states [
    loading when is-loading: {
        opacity: 0.5;
    }
]
```

### 3. **Temas Dinâmicos**
```slint
global Theme {
    in-out property <bool> dark-mode: true;
    out property <color> background: dark-mode ? #0a0a14 : #ffffff;
}
```

### 4. **Componentes Avançados**
- TabView
- Sidebar
- Toast notifications
- Progress indicators

---

## 📚 Recursos

- [Slint Documentation](https://slint.dev/docs)
- [Component Gallery](https://slint.dev/snapshots/master/docs/slint/src/advanced/generated_code)
- [ReactJS Patterns](https://react.dev/learn/thinking-in-react) (inspiração)

---

**🎉 Refatoração concluída com sucesso!**  
Compilado em **6.32s** sem erros.
