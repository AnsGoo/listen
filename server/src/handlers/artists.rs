use actix_web::{web, HttpResponse, Responder, HttpRequest, HttpMessage};
use super::super::{models, AppState, services};
use crate::services::artists::model::ArtistQueryViewObject;
use crate::handlers::ApiResponse;

// 获取歌手列表
pub async fn get_artists(
    query: web::Query<ArtistQueryViewObject>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    // 将ViewObject转换为DataObject
    let data_query = models::ArtistQueryData {
        id: query.id.clone(),
        name: query.name.clone(),
        nationality: query.nationality.clone(),
        sex: query.sex.clone(),
        page: query.page.map(|p| p as u64),
        page_size: query.limit.map(|l| l as u64),
    };

    let artists = services::artists::get_artists_service(data_query, state.config.artist_repo.clone())
        .await
        .map_err(|e| {
            log::error!("Service error: {:?}", e);
            actix_web::error::ErrorInternalServerError(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some(format!("Failed to fetch artists: {:?}", e)),
            })
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(artists),
        message: Some("Artists fetched successfully".to_string()),
    }))
}

// 根据ID获取歌手详情
pub async fn get_artist_by_id(
    artist_id: web::Path<uuid::Uuid>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let artist = services::artists::get_artist_by_id_service(artist_id.into_inner(), state.config.artist_repo.clone())
        .await
        .map_err(|e| {
            log::error!("Service error: {:?}", e);
            actix_web::error::ErrorInternalServerError(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some(format!("Failed to fetch artist: {:?}", e)),
            })
        })?;

    Ok(match artist {
        Some(artist) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(artist),
            message: Some("Artist fetched successfully".to_string()),
        }),
        None => HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Artist not found".to_string()),
        }),
    })
}

// 创建新歌手
pub async fn create_artist(
    data: web::Json<services::artists::model::CreateArtistViewObject>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<impl Responder, actix_web::Error> {
    // 将ViewObject转换为DataObject
    // 将ViewObject转换为DataObject
    let data_object = models::CreateArtistData {
        name: data.name.clone(),
        nationality: data.nationality.clone(),
        birth_date: data.birth_date.clone(),
        avatar: data.avatar.clone(),
        sex: data.sex.clone(),
        created_by: req.extensions_mut().get::<String>().cloned().unwrap_or("system".to_string()),
    };

    let artist = services::artists::create_artist_service(data_object, state.config.artist_repo.clone())
        .await
        .map_err(|e| {
            log::error!("Service error: {:?}", e);
            actix_web::error::ErrorInternalServerError(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some(format!("Failed to create artist: {:?}", e)),
            })
        })?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(artist),
        message: Some("Artist created successfully".to_string()),
    }))
}