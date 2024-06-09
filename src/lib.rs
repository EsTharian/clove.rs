//! Clove.rs is a Rust web framework for building web applications. It's powered by Axum.

mod modules;

pub use modules::*;

use colored::Colorize;

pub async fn bootstrap(routes: core::Router) {
    logger::fmt::fmt().with_max_level(log::Level::DEBUG).init();

    log::debug!("Logger initialized");

    // Load environment variables from .env file and production or development environment variables from .env.production or .env.development file
    // Override power: .env.local > .env.development > .env.production > .env
    let is_env_loaded = core::env::dotenv_override().ok().is_some()
        || core::env::from_filename_override(".env.production")
            .ok()
            .is_some()
        || core::env::from_filename_override(".env.development")
            .ok()
            .is_some()
        || core::env::from_filename_override(".env.local")
            .ok()
            .is_some();

    if !is_env_loaded {
        log::error!("Failed to load any dot-env file!\n-------------> {}\n-------------> {}{}",
            "Please create a .env or/and .env.[local|development|production] file in the root of your project.".red().bold(),
            "TIP: ".blue().bold(),
            "You can copy the .env.example file and rename it to .env.".cyan());

        std::process::exit(1);
    }

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    core::serve(listener, routes).await.unwrap();
}
