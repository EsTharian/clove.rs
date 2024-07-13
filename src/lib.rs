//! Clove.rs is a Rust web framework for building web applications. It's powered with Axum.

mod modules;

pub use modules::*;

pub async fn bootstrap(provider: Vec<core::Provider>) {
    core::env::load_env();

    tracing::init();

    core::run_server(provider).await.unwrap();
}
