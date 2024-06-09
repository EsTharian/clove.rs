//! Clove.rs is a Rust web framework for building web applications. It's powered by Axum.

mod modules;

pub use modules::*;

use colored::Colorize;

pub fn init() {
    logger::fmt::fmt().with_max_level(log::Level::DEBUG).init();

    log::info!("Logger initialized");

    // Load environment variables from .env file and production or development environment variables from .env.production or .env.development file
    // Override power: .env.local > .env.development > .env.production > .env
    let is_env_loaded = env::dotenv_override().ok().is_some()
        || env::from_filename_override(".env.production")
            .ok()
            .is_some()
        || env::from_filename_override(".env.development")
            .ok()
            .is_some()
        || env::from_filename_override(".env.local").ok().is_some();

    if !is_env_loaded {
        log::error!("Failed to load any dot-env file!\n-------------> {}\n-------------> {}{}",
            "Please create a .env or/and .env.[local|development|production] file in the root of your project.".red().bold(),
            "TIP: ".blue().bold(),
            "You can copy the .env.example file and rename it to .env.".cyan());

        std::process::exit(1);
    }
}
