pub mod model;
use crate::models;
use sea_orm::DbErr; 
use uuid::Uuid;
use std::sync::Arc;
use crate::models::song::SongRepository;
use crate::models::album::AlbumRepository;
use crate::models::artist::ArtistRepository;

// 获取歌曲列表服务
pub async fn get_songs_service(
    query: models::SongQueryParams,
    song_repo: Arc<dyn SongRepository + Send + Sync>
) -> Result<Vec<models::Song>, DbErr> {
    song_repo.find_all(&query).await
}

// 根据ID获取歌曲详情服务
pub async fn get_song_by_id_service(
    song_id: Uuid,
    song_repo: Arc<dyn SongRepository + Send + Sync>
) -> Result<Option<models::Song>, DbErr> {
    song_repo.find_by_id(song_id).await
}

// 创建歌曲服务
pub async fn create_song_service(
    data: models::CreateSongRequest,
    song_repo: Arc<dyn SongRepository + Send + Sync>,
    album_repo: Arc<dyn AlbumRepository + Send + Sync>,
    artist_repo: Arc<dyn ArtistRepository + Send + Sync>
) -> Result<models::Song, DbErr> {
    // 验证专辑是否存在
    let album_exists = album_repo.find_by_id(data.album_id).await?;
    if album_exists.is_none() {
        return Err(DbErr::Custom("Album not found".to_string()));
    }

    // 验证歌手是否存在
    let artist_exists = artist_repo.find_by_id(data.artist_id).await?;
    if artist_exists.is_none() {
        return Err(DbErr::Custom("Artist not found".to_string()));
    }

    song_repo.create(&data).await
}