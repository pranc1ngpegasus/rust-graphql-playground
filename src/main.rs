use axum::Router;
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "String::default")]
    pub port: String,
    #[serde(default = "String::default")]
    pub writer_database_url: String,
    #[serde(default = "String::default")]
    pub reader_database_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config =
        envy::from_env::<Config>().map_err(|e| anyhow::anyhow!("failed to load config: {}", e))?;

    let listener = TcpListener::bind(format!("0.0.0.0:{}", &config.port))
        .await
        .map_err(|e| anyhow::anyhow!("failed to bind port: {}", e))?;

    let router = Router::new();

    axum::serve(listener, router)
        .await
        .map_err(|e| anyhow::anyhow!("failed to run axum router: {}", e))?;

    Ok(())
}
