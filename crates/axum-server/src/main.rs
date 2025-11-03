mod api_keys;

use axum::{Router, routing::post};
use db::{queries, PgPool};

mod user_management;

fn make_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", post(|| async { "Hello, World!" }))
        .route("/register", post(user_management::register::register))
        .route("/login", post(user_management::login::login))
        .with_state(pool)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::create_pool(&db_url).await?;
    
    // Test database connection
    if let Some(u) = queries::user::get_by_id(&pool, 1).await? {
        println!("User: {:?}", u);
    }

    println!("Server starting...");

    // run the axum service
    // Fix: Use correct address format for TcpListener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    let app = make_router(pool);
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await?;
    
    Ok(())
}
