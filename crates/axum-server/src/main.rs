mod api_keys;

use axum::{Router, routing::post};
use db::queries;

mod user_management;

fn make_router(pool: deadpool_postgres::Pool) -> Router {
    Router::new()
        .route("/", post(|| async { "Hello, World!" }))
        .route("/register", post(user_management::register::register))
        .route("/login", post(user_management::login::login))
        .with_state(pool)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::create_pool(&db_url);
    
    // Test database connection
    let client = pool.get().await.unwrap();
    let u = queries::user::user().bind(&client, &1).one().await.unwrap();
    println!("User: {:?}", u);

    println!("Server starting...");

    // run the axum service
    // Fix: Use correct address format for TcpListener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    let app = make_router(pool);
    println!("Server running on http://127.0.0.1:3000");
    axum::serve(listener, app).await?;
    
    Ok(())
}
