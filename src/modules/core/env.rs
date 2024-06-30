use crate::tracing;
use colored::Colorize;
pub use dotenvy::*;

/// Load environment variables from `.env` file and debug (development), release (production) or test environment variables
/// from `.env.development`, `.env.production` or `.env.test` file depending on compilation profile.
///
/// Override power: `.env.test` > `.env.local` > `.env.[development|production]` > `.env`
pub fn load_env() {
    let is_env_loaded = dotenv_override().is_ok()
        | load_profile_env()
        | from_filename_override(".env.local").is_ok();

    #[cfg(test)]
    let is_env_loaded = is_env_loaded | from_filename_override(".env.test").is_ok();

    if !is_env_loaded {
        tracing::subscriber::fmt::fmt()
            .with_max_level(tracing::Level::ERROR)
            .init();

        tracing::error!("Failed to load any dot-env file!\n-------------> {}\n-------------> {}{}",
            "Please create a .env or/and .env.[local|development|production] file in the root of your project.".red().bold(),
            "TIP: ".blue().bold(),
            "You can copy the .env.example file and rename it to .env.".cyan());

        std::process::exit(1);
    }
}

#[cfg(debug_assertions)]
fn load_profile_env() -> bool {
    from_filename_override(".env.debug").is_ok()
}

#[cfg(not(debug_assertions))]
fn load_profile_env() -> bool {
    from_filename_override(".env.release").is_ok()
}
