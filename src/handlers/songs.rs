use actix_web::{web, HttpResponse, Responder};
use crate::{models, AppState, services};
use services::songs::model::SongQueryViewObject;
use crate::handlers::ApiResponse;

// 获取歌曲列表（支持按专辑、歌手、流派和音质筛选）
pub async fn get_songs(
    query: web::Query<SongQueryViewObject>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    // 将ViewObject转换为DataObject
    let data_query = models::SongQueryParams {
        album_id: query.album_id,
        artist_id: query.artist_id,
        genre: query.genre.clone(),
        quality: query.quality.clone(),
        page: query.page.map(|p| p as u64),
        page_size: query.limit.map(|l| l as u64),
    };

    let songs = services::songs::get_songs_service(data_query, state.config.song_repo.clone())
        .await
        .map_err(|e| {
            log::error!("Service error: {:?}", e);
            actix_web::error::ErrorInternalServerError(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some(format!("Failed to fetch songs: {:?}", e)),
            })
        })?;

    Ok(HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: Some(songs),
        message: Some("Songs fetched successfully".to_string()),
    }))
}

// 根据ID获取歌曲详情
pub async fn get_song_by_id(
    song_id: web::Path<uuid::Uuid>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let song = services::songs::get_song_by_id_service(song_id.into_inner(), state.config.song_repo.clone())
        .await
        .map_err(|e| {
            log::error!("Service error: {:?}", e);
            actix_web::error::ErrorInternalServerError(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some(format!("Failed to fetch song: {:?}", e)),
            })
        })?;

    Ok(match song {
        Some(song) => HttpResponse::Ok().json(ApiResponse {
            success: true,
            data: Some(song),
            message: Some("Song fetched successfully".to_string()),
        }),
        None => HttpResponse::NotFound().json(ApiResponse::<()> {
            success: false,
            data: None,
            message: Some("Song not found".to_string()),
        }),
    })
}

// 创建新歌曲
pub async fn create_song(
    data: web::Json<services::songs::model::CreateSongViewObject>,
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    // 将ViewObject转换为DataObject
    let data_object = models::CreateSongRequest {
        title: data.title.clone(),
        album_id: data.album_id,
        artist_id: data.artist_id.clone(),
        genre: data.genre.clone(),
        duration: data.duration,
        quality: data.quality.clone(),
        file_path: data.file_path.clone(),
    };

    let song = services::songs::create_song_service(data_object, state.config.song_repo.clone(), state.config.album_repo.clone(), state.config.artist_repo.clone())
        .await
        .map_err(|e| {
            log::error!("Service error: {:?}", e);
            let message = match e.to_string().as_str() {
                "Album not found" | "Artist not found" => e.to_string(),
                _ => "Failed to create song".to_string(),
            };
            actix_web::error::ErrorInternalServerError(ApiResponse::<()> {
                success: false,
                data: None,
                message: Some(message),
            })
        })?;

    Ok(HttpResponse::Created().json(ApiResponse {
        success: true,
        data: Some(song),
        message: Some("Song created successfully".to_string()),
    }))
}