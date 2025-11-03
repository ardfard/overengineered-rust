pub mod queries;

pub use sqlx::PgPool;

pub async fn create_pool(database_url: &str) -> sqlx::Result<PgPool> {
    sqlx::PgPool::connect(database_url).await
}