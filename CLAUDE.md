# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

PCCLEAN-APP is a Rust-based disk cleaning application built with **Domain-Driven Design (DDD)**, **Clean Architecture**, and **SOLID principles**. It features a cyberpunk-themed GUI using egui/wgpu with multiple theme support.

## Build and Development Commands

### Running the Application
```bash
# Debug mode
cargo run

# Release mode (optimized)
cargo run --release
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality
```bash
# Check for errors and warnings
cargo clippy

# Format code
cargo fmt

# Check formatting without modifying
cargo fmt --check
```

### Build
```bash
# Build debug
cargo build

# Build release with optimizations
cargo build --release
```

## Architecture

The codebase follows a strict layered architecture with clear dependency rules:

```
src/
├── domain/              # Core business logic (NO external dependencies)
│   ├── entities/        # CleanableItem, CleanableCategory, ScanResult
│   ├── value_objects/   # CategoryType enum
│   └── repositories/    # Trait definitions (ScannerRepository, CleanerRepository)
│
├── application/         # Use cases orchestrating domain logic
│   └── use_cases/       # ScanSystemUseCase, CleanSelectedCategoriesUseCase
│
├── infrastructure/      # Concrete implementations
│   └── repositories/    # FileSystemScanner, FileSystemCleaner
│
├── presentation/        # UI layer (egui)
│   ├── app.rs          # Main application & dependency injection
│   ├── widgets/        # Reusable UI components
│   └── themes/         # Theme system with 6 themes
│
└── shared/             # Cross-layer utilities
    └── formatters.rs   # format_bytes, etc.
```

### Dependency Flow
- **Domain** has ZERO dependencies on other layers
- **Application** depends only on Domain (via traits)
- **Infrastructure** implements Domain traits
- **Presentation** depends only on Application use cases
- **Shared** is used by all layers

### Key Architectural Patterns
- **Repository Pattern**: Abstract data access via traits
- **Use Case Pattern**: Each business operation is a separate use case
- **Dependency Injection**: Dependencies injected in [main.rs:47-53](src/main.rs#L47-L53)
- **Clean Architecture**: Strict separation of concerns

## Adding New Features

### 1. Adding a New Cleaning Category

**Step 1**: Add to CategoryType enum in [src/domain/value_objects/category_type.rs](src/domain/value_objects/category_type.rs)
```rust
pub enum CategoryType {
    // ... existing variants
    BrowserCache,  // New category
}
```

**Step 2**: Implement scanner in [src/infrastructure/repositories/filesystem_scanner_repository.rs](src/infrastructure/repositories/filesystem_scanner_repository.rs)
```rust
fn scan_browser_cache(&self) -> Result<Vec<CleanableItem>> {
    // Implementation
}
```

**Step 3**: Add match arm in `ScannerRepository::scan_category()`

The UI automatically adapts to new categories - no presentation changes needed.

### 2. Adding a New Use Case

Create a new struct in [src/application/use_cases/](src/application/use_cases/):
```rust
pub struct NewUseCase {
    repository: Arc<dyn SomeRepository>,
}

impl NewUseCase {
    pub fn execute(&self, params: Params) -> Result<Output> {
        // Business logic orchestration
    }
}
```

**IMPORTANT**: Use cases should:
- Accept repositories via constructor (dependency injection)
- Contain ONLY orchestration logic (no business rules)
- Return domain entities
- Be fully unit testable

### 3. Adding a New Theme

Create file in [src/presentation/themes/](src/presentation/themes/):
```rust
pub fn apply(ctx: &egui::Context) {
    // Configure egui::Style and egui::Visuals
}
```

Add variant to `ThemeType` enum and update `ThemeManager`.

See [TEMAS.md](TEMAS.md) for theme system documentation.

## Domain-Driven Design Guidelines

### Domain Layer Rules
- **Entities** should contain behavior, not just data
- **Value Objects** must be immutable
- NO framework dependencies (egui, tokio, etc.)
- NO I/O operations (file system, network, etc.)
- Business rules belong HERE

### Testing
- Domain entities have unit tests embedded (see [src/domain/value_objects/category_type.rs:47-61](src/domain/value_objects/category_type.rs#L47-L61))
- Use cases are tested with mock repositories
- Infrastructure tests verify actual I/O operations

## Common Patterns in This Codebase

### Scanning System
1. UI calls `ScanSystemUseCase::execute()`
2. Use case delegates to `ScannerRepository::scan_system()`
3. Repository scans each category via `scan_category(CategoryType)`
4. Returns `ScanResult` with aggregated `CleanableCategory` instances

### Cleaning System
1. UI calls `CleanSelectedCategoriesUseCase::execute(selected_indices)`
2. Use case retrieves categories from `ScanResult`
3. Delegates to `CleanerRepository::clean_category()`
4. Returns count of cleaned items

### Threading Model
- Scanning runs in background thread (see [src/presentation/app.rs:70-85](src/presentation/app.rs#L70-L85))
- Cleaning uses thread + channel for progress updates
- Results shared via `Arc<Mutex<Option<ScanResult>>>`

## Key Files to Understand

- [src/main.rs](src/main.rs) - Entry point with dependency injection setup
- [src/presentation/app.rs](src/presentation/app.rs) - Main application loop and state
- [src/domain/repositories/scanner_repository.rs](src/domain/repositories/scanner_repository.rs) - Scanner trait definition
- [src/infrastructure/repositories/filesystem_scanner_repository.rs](src/infrastructure/repositories/filesystem_scanner_repository.rs) - Actual scanning implementation
- [src/domain/value_objects/category_type.rs](src/domain/value_objects/category_type.rs) - All available categories

## Critical Constraints

1. **NEVER** add infrastructure dependencies to Domain layer
2. **ALWAYS** use `Result<T>` for operations that can fail
3. **NEVER** mix business logic in UI code
4. **ALWAYS** write unit tests for new domain entities
5. **NEVER** break the dependency flow (Domain ← Application ← Infrastructure/Presentation)

## SOLID Principles Applied

- **Single Responsibility**: Each use case handles ONE operation
- **Open/Closed**: Extend via new Repository implementations, not modifications
- **Liskov Substitution**: All `ScannerRepository` implementations are interchangeable
- **Interface Segregation**: Separate traits for Scanner vs Cleaner
- **Dependency Inversion**: Use cases depend on abstractions (traits), not concretions

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) - Detailed architecture explanation
- [EXTENSIBILITY_GUIDE.md](EXTENSIBILITY_GUIDE.md) - Step-by-step feature addition guides
- [TEMAS.md](TEMAS.md) - Theme system documentation
- [README.md](README.md) - User-facing documentation

## Platform Notes

- Primary platform: macOS (paths like `~/Library/Logs`)
- Docker scanning requires Docker CLI installed
- Uses `walkdir` for recursive directory traversal
- GPU-accelerated rendering via wgpu backend
