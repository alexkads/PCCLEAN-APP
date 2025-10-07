// Camadas da aplicação seguindo DDD e Clean Architecture
mod domain;
mod application;
mod infrastructure;
mod presentation;
mod shared;

use presentation::run_app;

/// Ponto de entrada da aplicação.
/// Configura e inicializa a interface gráfica com Slint.
fn main() -> Result<(), slint::PlatformError> {
    run_app()
}
