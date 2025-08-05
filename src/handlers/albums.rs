use actix_web::{web, HttpResponse, Responder};
// 移除SQLx引用
use super::super::{models, AppState};
use crate::services::{self, albums::model::AlbumQueryViewObject};
use crate::handlers::ApiResponse;

// 获取专辑列表（支持按歌手、名称和发行日期筛选）
pub async fn get_albums(
    query: web::Query<AlbumQueryViewObject>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    // 将ViewObject转换为DataObject
    let data_object = AlbumQueryViewObject {
        id: query.id,
        artist_id: query.artist_id,
        name: query.name.clone(),
        release_year: query.release_year,
        page: query.page.map(|p| p as u32),
        page_size: query.page_size.map(|l| l as u32),
    };

    let albums = services::albums::get_albums_service(data_object, state.config.album_repo.clone())
        .await
        .map_err(|e| {
            log::error!("Service error: {:?}", e);
            actix_web::error::ErrorInternalServerError(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some("Failed to fetch albums".to_string()),
            })
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(albums),
        message: Some("Albums fetched successfully".to_string()),
    }))
}

// 根据ID获取专辑详情
pub async fn get_album_by_id(
    album_id: web::Path<uuid::Uuid>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let album_id = album_id.into_inner();

    let album = services::albums::get_album_by_id_service(album_id, state.config.album_repo.clone())
        .await
        .map_err(|e| {
            log::error!("Service error: {:?}", e);
            let message = match e.to_string().as_str() {
                "Album not found" => e.to_string(),
                _ => "Failed to fetch album".to_string(),
            };
            actix_web::error::ErrorInternalServerError(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some(message),
            })
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(album),
        message: Some("Album fetched successfully".to_string()),
    }))
}

// 创建新专辑
pub async fn create_album(
    data: web::Json<services::albums::model::CreateAlbumViewObject>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    // 将ViewObject转换为DataObject
    let data_object = data.into_inner();

    let album = services::albums::create_album_service(data_object, state.config.album_repo.clone(), state.config.artist_repo.clone())
        .await
        .map_err(|e| {
            log::error!("Service error: {:?}", e);
            let message = match e.to_string().as_str() {
                "Artist_ not found" => e.to_string(),
                _ => "Failed to create album".to_string(),
            };
            actix_web::error::ErrorInternalServerError(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some(message),
            })
        })?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(album),
        message: Some("Album created successfully".to_string()),
    }))
}