pub mod query_organization_by_id;

use sqlx::PgConnection;
use std::future::Future;
use std::pin::Pin;

pub trait Repository<Input, Output>: Send + Sync + 'static {
    fn handle<'a>(
        &'a self,
        conn: &'a mut PgConnection,
        input: Input,
    ) -> Pin<Box<dyn Future<Output = anyhow::Result<Output>> + Send + 'a>>;
}
