use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::EnvFilter;

pub fn init() -> anyhow::Result<()> {
    let format = tracing_subscriber::fmt::layer()
        .json()
        .with_file(true)
        .with_level(true)
        .with_target(true);

    let env_filter = EnvFilter::builder()
        .with_env_var("LOG_LEVEL")
        .try_from_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::Registry::default()
        .with(format)
        .with(env_filter);

    Ok(())
}
