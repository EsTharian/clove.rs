pub use axum::*;

pub mod env;

pub struct Provider {
    pub name: &'static str,
    pub routes: Router,
}

pub async fn run_server(provider: Vec<Provider>) -> Result<(), std::io::Error> {
    const DEFAULT_HOST: &str = "0";
    const DEFAULT_PORT: &str = "3000";

    let provided_routes = provider.iter().fold(Router::new(), |acc, provided| {
        acc.merge(provided.routes.clone())
    });

    let host = std::env::var("APP_HOST")
        .unwrap_or(DEFAULT_HOST.to_string())
        .to_string();

    let port = std::env::var("APP_PORT")
        .unwrap_or(DEFAULT_PORT.to_string())
        .to_string();

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    serve(listener, provided_routes)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install CTRL+C signal handler");
        })
        .await
}
