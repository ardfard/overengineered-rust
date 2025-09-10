
use axum::{extract::State, http::StatusCode, response::Json as ResponseJson, Json};
use serde_json::Value;
use crate::user_management::models::{AuthResponse, RegisterRequest, UserResponse};
use db::queries;

pub async fn register(
    State(pool): State<deadpool_postgres::Pool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<ResponseJson<Value>, (StatusCode, ResponseJson<Value>)> {
    // Basic validation
    if payload.email.is_empty() || payload.password.is_empty() || payload.username.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(serde_json::json!({
                "success": false,
                "message": "All fields are required",
                "data": null
            })),
        ));
    }

    // Email validation (basic)
    if !payload.email.contains('@') {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(serde_json::json!({
                "success": false,
                "message": "Invalid email format",
                "data": null
            })),
        ));
    }

    // Password validation (basic)
    if payload.password.len() < 6 {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(serde_json::json!({
                "success": false,
                "message": "Password must be at least 6 characters",
                "data": null
            })),
        ));
    }

    // Get database connection
    let client = match pool.get().await {
        Ok(client) => client,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(serde_json::json!({
                    "success": false,
                    "message": "Database connection failed",
                    "data": null
                })),
            ));
        }
    };

    // Check if email already exists
    match queries::user::get_by_email().bind(&client, &payload.email).one().await {
        Ok(_) => {
            return Err((
                StatusCode::CONFLICT,
                ResponseJson(serde_json::json!({
                    "success": false,
                    "message": "Email already exists",
                    "data": null
                })),
            ));
        }
        Err(_) => {
            // Email doesn't exist, continue with registration
        }
    }

    // Hash the password
    let password_hash = match bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(serde_json::json!({
                    "success": false,
                    "message": "Password hashing failed",
                    "data": null
                })),
            ));
        }
    };

    // Insert user into database
    let user = match queries::user::insert_user()
        .bind(&client, &payload.email, &payload.username, &password_hash)
        .one()
        .await
    {
        Ok(user) => user,
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(serde_json::json!({
                    "success": false,
                    "message": "Failed to create user",
                    "data": null
                })),
            ));
        }
    };

    let user_response = UserResponse {
        id: user.id,
        email: user.email,
        username: user.username,
        created_at: user.created_at.format(&time::format_description::well_known::Rfc3339).unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_string()),
    };

    let auth_response = AuthResponse {
        user: user_response,
        token: "demo_jwt_token_here".to_string(), // In real app, generate JWT
    };

    Ok(ResponseJson(serde_json::json!({
        "success": true,
        "message": "User registered successfully",
        "data": auth_response
    })))
}