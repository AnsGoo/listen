pub mod model;
use chrono::NaiveDate;

use self::model::*;
use crate::models::{self, CreateAlbumData};
use std::fmt;
use std::sync::Arc;
use crate::models::album::AlbumRepository;
use crate::models::artist::ArtistRepository;
use crate::models::AlbumQueryData;

#[derive(Debug)]
pub enum AlbumServiceError {
    DatabaseError(sea_orm::DbErr),
    ArtistNotFound,
    AlbumNotFound,
}

impl fmt::Display for AlbumServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlbumServiceError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            AlbumServiceError::ArtistNotFound => write!(f, "Artist not found"),
            AlbumServiceError::AlbumNotFound => write!(f, "Album not found"),
        }
    }
}

/// 获取专辑列表服务
pub async fn get_albums_service(
    query: AlbumQueryViewObject,
    album_repo: Arc<dyn AlbumRepository + Send + Sync>
) -> Result<Vec<AlbumDetailViewObject>, AlbumServiceError> {

    let data_object = models::AlbumQueryData {
        artist_id: query.artist_id,
        name: query.name.clone(),
        release_date: query.release_year.map(|year| NaiveDate::from_ymd_opt(year, 1, 1).unwrap_or_default()),
        page: query.page.map(|p| p as u32),
        page_size: query.page_size.map(|l| l as u32),
    };
    let albums = album_repo.find_all(&data_object)
        .await
        .map_err(AlbumServiceError::DatabaseError)?;

    // 将DataObject转换为ViewObject
    let album_view_objects = albums
        .into_iter()
        .map(|album| AlbumDetailViewObject {
            id: album.id,
            name: album.name,
            artist_id: album.artist_id,
            cover_image: album.cover_image.unwrap_or_default(),
            release_date: album.release_date,
            description: album.description,
            created_at: album.created_at.into(),
            updated_at: album.updated_at.into(),
        })
        .collect();

    Ok(album_view_objects)
}

/// 根据ID获取专辑详情服务
pub async fn get_album_by_id_service(
    album_id: uuid::Uuid,
    album_repo: Arc<dyn AlbumRepository + Send + Sync>
) -> Result<Option<AlbumDetailViewObject>, AlbumServiceError> {
    let album = album_repo.find_by_id(album_id)
        .await
        .map_err(AlbumServiceError::DatabaseError)?;

    Ok(album.map(|album| AlbumDetailViewObject {
        id: album.id,
        name: album.name,
        artist_id: album.artist_id,
        cover_image: album.cover_image.unwrap_or_default(),
        release_date: album.release_date,
        description: album.description,
        created_at: album.created_at.into(),
        updated_at: album.updated_at.into(),
    }))
}

/// 创建专辑服务
pub async fn create_album_service(
    data: model::CreateAlbumViewObject,
    album_repo: Arc<dyn AlbumRepository + Send + Sync>,
    artist_repo: Arc<dyn ArtistRepository + Send + Sync>
) -> Result<AlbumDetailViewObject, AlbumServiceError> {
    // 验证歌手是否存在
    let artist_exists = artist_repo.find_by_id(data.artist_id)
        .await
        .map_err(AlbumServiceError::DatabaseError)?;

    if artist_exists.is_none() {
        return Err(AlbumServiceError::ArtistNotFound);
    }

    let data = CreateAlbumData {
        name: data.name.clone(),
        artist_id: data.artist_id,
        cover_image: Some(data.cover_image),
        release_date: data.release_date,
        description: data.description.clone(),
        genre: data.genre.clone(),
    };
    // 创建专辑
    let album = album_repo.create(&data)
        .await
        .map_err(AlbumServiceError::DatabaseError)?;

    Ok(AlbumDetailViewObject {
        id: album.id,
        name: album.name,
        artist_id: album.artist_id,
        cover_image: album.cover_image.unwrap_or_default(),
        release_date: album.release_date,
        description: album.description,
        created_at: album.created_at.into(),
        updated_at: album.updated_at.into(),
    })
}