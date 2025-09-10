use axum::{extract::State, http::StatusCode, response::Json as ResponseJson, Json};
use serde_json::Value;
use crate::user_management::models::{AuthResponse, LoginRequest, UserResponse};
use db::queries;

// For now, we'll use a simple hardcoded approach
// In a real application, you'd use a proper database and JWT tokens

pub async fn login(
    State(pool): State<deadpool_postgres::Pool>,
    Json(payload): Json<LoginRequest>,
) -> Result<ResponseJson<Value>, (StatusCode, ResponseJson<Value>)> {
    // Basic validation
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            ResponseJson(serde_json::json!({
                "success": false,
                "message": "Email and password are required",
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

    // Get user by email
    let user = match queries::user::get_by_email_auth()
        .bind(&client, &payload.email)
        .one()
        .await
    {
        Ok(user) => user,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                ResponseJson(serde_json::json!({
                    "success": false,
                    "message": "Invalid credentials",
                    "data": null
                })),
            ));
        }
    };

    // Verify password
    match bcrypt::verify(&payload.password, &user.password_hash) {
        Ok(true) => {
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
                "message": "Login successful",
                "data": auth_response
            })))
        }
        Ok(false) | Err(_) => {
            Err((
                StatusCode::UNAUTHORIZED,
                ResponseJson(serde_json::json!({
                    "success": false,
                    "message": "Invalid credentials",
                    "data": null
                })),
            ))
        }
    }
}
