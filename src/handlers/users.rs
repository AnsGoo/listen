use actix_web::web::Query;
use actix_web:: {web, HttpResponse, Responder};

use crate::services::users::{UserQueryViewObject, get_users_service};
use crate::AppState;
use crate::handlers::ApiResponse;

pub async fn get_users(query: Query<UserQueryViewObject>, state: web::Data<AppState>) ->Result<impl Responder, actix_web::Error> { 
    
    let query_data = query.into_inner();
    let users = get_users_service(query_data, state.config.user_repo.clone()).await
    .map_err(|e| actix_web::error::ErrorBadRequest(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some(format!("Failed to fetch users: {:?}", e)),
            }))?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(users),
        message: Some("users fetched successfully".to_string()),
    }))
}