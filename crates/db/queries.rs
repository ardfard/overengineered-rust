use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserWithPassword {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
}

pub mod user {
    use super::*;
    use sqlx::PgPool;

    pub async fn get_by_id(pool: &PgPool, id: i32) -> sqlx::Result<Option<User>> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, COALESCE(username, '') as username, created_at::timestamptz as created_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn insert_user(
        pool: &PgPool,
        email: &str,
        username: &str,
        password_hash: &str,
    ) -> sqlx::Result<User> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (email, username, password_hash)
            VALUES($1, $2, $3)
            RETURNING id, email, COALESCE(username, '') as username, created_at::timestamptz as created_at
            "#,
        )
        .bind(email)
        .bind(username)
        .bind(password_hash)
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_openid_sub(pool: &PgPool, openid_sub: &str) -> sqlx::Result<Option<User>> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, COALESCE(username, '') as username, created_at::timestamptz as created_at
            FROM users
            WHERE openid_sub = $1
            "#,
        )
        .bind(openid_sub)
        .fetch_optional(pool)
        .await
    }

    pub async fn get_by_email(pool: &PgPool, email: &str) -> sqlx::Result<Option<User>> {
        sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, COALESCE(username, '') as username, created_at::timestamptz as created_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await
    }

    pub async fn get_by_email_auth(
        pool: &PgPool,
        email: &str,
    ) -> sqlx::Result<Option<UserWithPassword>> {
        sqlx::query_as::<_, UserWithPassword>(
            r#"
            SELECT id, email, COALESCE(username, '') as username, password_hash, created_at::timestamptz as created_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await
    }

    pub async fn count_users(pool: &PgPool) -> sqlx::Result<i64> {
        let row = sqlx::query!(r#"SELECT count(id) as count FROM users"#)
            .fetch_one(pool)
            .await?;
        Ok(row.count.unwrap_or(0))
    }
}
