use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use loco_rs::{auth::jwt::JWT, prelude::*};

/// Middleware that checks for a valid JWT token in cookies.
/// If the token is missing or invalid, redirects to the login page.
/// Use this for browser-facing routes that should redirect instead of returning 401.
pub async fn browser_auth_middleware(
    State(ctx): State<AppContext>,
    request: Request,
    next: Next,
) -> Response {
    // Extract token from cookie
    let token = request
        .headers()
        .get("cookie")
        .and_then(|cookie_header| cookie_header.to_str().ok())
        .and_then(|cookies| {
            cookies
                .split(';')
                .map(|s| s.trim())
                .find(|s| s.starts_with("token="))
                .map(|s| s.trim_start_matches("token=").to_string())
        });

    let Some(token) = token else {
        tracing::debug!("No token cookie found, redirecting to login");
        return Redirect::to("/login").into_response();
    };

    if token.is_empty() {
        tracing::debug!("Empty token cookie, redirecting to login");
        return Redirect::to("/login").into_response();
    }

    // Validate the JWT token
    let jwt_config = match ctx.config.get_jwt_config() {
        Ok(config) => config,
        Err(e) => {
            tracing::error!("Failed to get JWT config: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    match JWT::new(&jwt_config.secret).validate(&token) {
        Ok(claims) => {
            if claims.claims.pid.is_empty() {
                tracing::debug!("Token has empty pid, redirecting to login");
                return Redirect::to("/login").into_response();
            }
            // Token is valid, proceed with the request
            next.run(request).await
        }
        Err(e) => {
            tracing::debug!("Invalid token: {}, redirecting to login", e);
            Redirect::to("/login").into_response()
        }
    }
}

