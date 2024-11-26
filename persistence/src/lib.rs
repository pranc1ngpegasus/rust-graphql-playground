use sqlx::pool::PoolConnection;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, PgConnection, PgPool, Postgres};
use std::future::Future;
use std::pin::Pin;
use tracing::Instrument;

pub trait ReaderPoolExt {
    fn acquire<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<PoolConnection<Postgres>>> + Send + 'a>>;
}

#[derive(Clone)]
pub struct ReaderPool(PgPool);

impl ReaderPool {
    pub fn try_new<'a>(
        dsn: &'a str,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Self>> + Send + 'a>> {
        Box::pin(async move {
            let pool = PgPoolOptions::new()
                .max_connections(10)
                .connect(dsn)
                .await
                .map_err(|e| anyhow::anyhow!("failed to connect to database: {e}"))?;

            Ok(Self(pool))
        })
    }
}

impl ReaderPoolExt for ReaderPool {
    #[tracing::instrument(name = "postgres.reader.acquire", skip_all)]
    fn acquire<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<PoolConnection<Postgres>>> + Send + 'a>> {
        Box::pin(async move {
            self.0
                .acquire()
                .await
                .map_err(|e| anyhow::anyhow!("failed to acquire connection: {:?}", e))
        })
    }
}

pub trait WriterPoolExt {
    fn acquire<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<PoolConnection<Postgres>>> + Send + 'a>>;
    fn with_tx<'a, F, T>(
        &'a self,
        inner: F,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<T>> + Send + 'a>>
    where
        for<'c> F: FnOnce(
                &'c mut PgConnection,
            ) -> std::pin::Pin<
                Box<dyn std::future::Future<Output = anyhow::Result<T>> + Send + 'c>,
            >
            + 'a
            + Send
            + Sync,
        T: Send;
}

#[derive(Clone)]
pub struct WriterPool(PgPool);

impl WriterPool {
    pub async fn try_new(dsn: &str) -> anyhow::Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(dsn)
            .await
            .map_err(|e| anyhow::anyhow!("failed to connect to database: {e}"))?;

        Ok(Self(pool))
    }
}

impl WriterPoolExt for WriterPool {
    #[tracing::instrument(name = "postgres.writer.acquire", skip_all)]
    fn acquire<'a>(
        &'a self,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<PoolConnection<Postgres>>> + Send + 'a>> {
        Box::pin(async move {
            self.0
                .acquire()
                .await
                .map_err(|e| anyhow::anyhow!("failed to acquire connection: {:?}", e))
        })
    }

    #[tracing::instrument(name = "postgres.writer.with_tx", skip_all)]
    fn with_tx<'a, F, T>(
        &'a self,
        inner: F,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<T>> + Send + 'a>>
    where
        for<'c> F: FnOnce(
                &'c mut PgConnection,
            ) -> std::pin::Pin<
                Box<dyn std::future::Future<Output = anyhow::Result<T>> + Send + 'c>,
            >
            + 'a
            + Send
            + Sync,
        T: Send,
    {
        Box::pin(async move {
            let mut conn = self
                .0
                .acquire()
                .instrument(tracing::info_span!("postgres.writer.with_tx.acquire"))
                .await
                .map_err(|e| anyhow::anyhow!("failed to acquire connection: {:?}", e))?;
            let mut tx = conn
                .begin()
                .instrument(tracing::info_span!("postgres.writer.with_tx.begin"))
                .await
                .map_err(|e| anyhow::anyhow!("failed to begin transaction: {:?}", e))?;
            let result = inner(&mut tx).await;

            match result {
                Ok(result) => {
                    tx.commit()
                        .instrument(tracing::info_span!("postgres.writer.with_tx.commit"))
                        .await
                        .map_err(|e| anyhow::anyhow!("failed to commit transaction: {:?}", e))?;

                    Ok(result)
                }
                Err(e) => {
                    tx.rollback()
                        .instrument(tracing::info_span!("postgres.writer.with_tx.rollback"))
                        .await
                        .map_err(|rollback_error| {
                            anyhow::anyhow!(
                                "failed to rollback transaction: {:?}, original error: {:?}",
                                rollback_error,
                                e
                            )
                        })?;

                    Err(e)
                }
            }
        })
    }
}
