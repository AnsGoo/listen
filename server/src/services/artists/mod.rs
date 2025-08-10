pub mod model;
use std::sync::Arc;
use crate::models::artist::{ArtistRepository, CreateArtistData, ArtistQueryData};
use crate::services::artists::model::ArtistDetailViewObject;
use std::fmt;

#[derive(Debug)]
pub enum ArtistServiceError {
    DatabaseError(sea_orm::DbErr),
    ArtistNotFound,
}

impl fmt::Display for ArtistServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArtistServiceError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            ArtistServiceError::ArtistNotFound => write!(f, "Artist not found"),
        }
    }
}

/// 获取歌手列表服务
pub async fn get_artists_service(
    query: ArtistQueryData,
    artist_repo: Arc<dyn ArtistRepository + Send + Sync>,
) -> Result<Vec<model::ArtistDetailViewObject>, ArtistServiceError> {
    let artists = artist_repo.find_all(&query)
        .await
        .map_err(ArtistServiceError::DatabaseError)?;

    // 将DataObject转换为ViewObject
    let artist_view_objects = artists
        .into_iter()
        .map(|artist| model::ArtistDetailViewObject {
              id: artist.id,
              name: artist.name,
              bio: artist.nationality.clone().unwrap_or_default(),
              birth_date: Some(artist.birth_date.unwrap_or_default()),
              avatar_url: Some(artist.avatar.clone().unwrap_or_default()),
              sex: Some(artist.sex.clone().unwrap_or_default()),
              created_at: artist.created_at.into(),
              updated_at: artist.updated_at.into(),
          })
        .collect();

    Ok(artist_view_objects)
}

/// 根据ID获取歌手详情服务
pub async fn get_artist_by_id_service(
    artist_id: uuid::Uuid,
    artist_repo: Arc<dyn ArtistRepository + Send + Sync>,
) -> Result<Option<ArtistDetailViewObject>, ArtistServiceError> {
    let artist = artist_repo.find_by_id(artist_id)
        .await
        .map_err(ArtistServiceError::DatabaseError)?;

    Ok(artist.map(|artist| ArtistDetailViewObject {
        id: artist.id,
        name: artist.name,
        bio: artist.nationality.clone().unwrap_or_default(),
        birth_date: Some(artist.birth_date.unwrap_or_default()),
        avatar_url: Some(artist.avatar.clone().unwrap_or_default()),
        sex: Some(artist.sex.clone().unwrap_or_default()),
        created_at: artist.created_at.into(),
        updated_at: artist.updated_at.into(),
    }))
}

/// 创建歌手服务
pub async fn create_artist_service(
    data: CreateArtistData,
    artist_repo: Arc<dyn ArtistRepository + Send + Sync>,
) -> Result<ArtistDetailViewObject, ArtistServiceError> {
    // 创建歌手
    let artist = artist_repo.create(&data)
        .await
        .map_err(ArtistServiceError::DatabaseError)?;

    Ok(ArtistDetailViewObject {
        id: artist.id,
        name: artist.name,
        bio: artist.nationality.clone().unwrap_or_default(),
        birth_date: Some(artist.birth_date.unwrap_or_default()),
        avatar_url: Some(artist.avatar.clone().unwrap_or_default()),
        created_at: artist.created_at,
        updated_at: artist.updated_at,
        sex: Some(artist.sex.clone().unwrap_or_default()),

    })
}