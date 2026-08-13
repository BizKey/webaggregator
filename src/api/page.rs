use askama::Template;
use sqlx::PgPool;
use std::future::Future;

pub trait Page: Template + Send + Sync + 'static {
    type Data: Send;

    fn load_data(pool: &PgPool) -> impl Future<Output = Result<Self::Data, sqlx::Error>> + Send;

    fn from_data(data: Self::Data, elapsed_ms: u128) -> Self;
}
