pub use tracing::*;

pub mod subscriber {
    pub use tracing_subscriber::*;
}

const DEFAULT_TRACE_LEVEL: &str = "warn";

pub fn init() {
    // TODO: "Log" term is not proper, tracing is used. Develop a better strategy for tracing.
    let trace_level = std::env::var("APP_TRACE_LEVEL").unwrap_or(DEFAULT_TRACE_LEVEL.to_string());

    match trace_level.as_str() {
        "error" => {
            subscriber::fmt::fmt().with_max_level(Level::ERROR).init();
        }
        "warn" => {
            subscriber::fmt::fmt().with_max_level(Level::WARN).init();
        }
        "info" => {
            subscriber::fmt::fmt().with_max_level(Level::INFO).init();
        }
        "debug" => {
            subscriber::fmt::fmt().with_max_level(Level::DEBUG).init();
        }
        "trace" => {
            subscriber::fmt::fmt().with_max_level(Level::TRACE).init();
        }
        _ => {}
    }

    debug!("Tracing initialized");
}
