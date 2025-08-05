use actix_web::{web, HttpResponse, Responder};
use super::super::{ AppState, services};
use actix_web_validator::Json;
use crate::services::auth::model::LoginViewObject;
use crate::handlers::ApiResponse;

// 用户注册
pub async fn register(
    data: Json<services::auth::model::RegisterViewObject>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let auth_response = services::auth::register_service(data.into_inner(), &state)
        .await
        .map_err(|e| {
            log::error!("Auth service error: {:?}", e);
            match e.to_string().as_str() {
                "Username already exists" => actix_web::http::StatusCode::BAD_REQUEST,
                _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            };
            actix_web::error::ErrorInternalServerError(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some(e.to_string()),
            })
        })?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(auth_response),
        message: Some("User registered successfully".to_string()),
    }))
}

// 用户登录
pub async fn login(
    data: web::Json<LoginViewObject>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let auth_response = services::auth::login_service(data.into_inner(), &state)
        .await
        .map_err(|e| {
            log::error!("Auth service error: {:?}", e);
            match e.to_string().as_str() {
                "Invalid username or password" => actix_web::http::StatusCode::UNAUTHORIZED,
                _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            };
            actix_web::error::ErrorInternalServerError(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some(e.to_string()),
            })
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(auth_response),
        message: Some("Login successful".to_string()),
    }))
}