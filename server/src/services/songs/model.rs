use serde::{Deserialize, Serialize}; 
use uuid::Uuid;
use chrono::{DateTime, Local};

// 歌曲查询视图对象
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SongQueryViewObject {
    pub album_id: Option<Uuid>,
    pub artist_id: Option<Uuid>,
    pub genre: Option<String>,
    pub quality: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

// 创建歌曲视图对象
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSongViewObject {
    pub title: String,
    pub album_id: Uuid,
    pub artist_id: Uuid,
    pub genre: Option<String>,
    pub duration: u32,
    pub quality: String,
    pub file_path: String,
    pub release_date: chrono::NaiveDate,
}

// 歌曲详情视图对象
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SongDetailViewObject {
    pub id: Uuid,
    pub title: String,
    pub album_id: Uuid,
    pub artist_id: Uuid,
    pub genre: String,
    pub duration: u32,
    pub quality: String,
    pub file_path: String,
    pub release_date: chrono::NaiveDate,
    #[serde(serialize_with = "crate::utils::date_time::format_datetime")]
    pub created_at: DateTime<Local>,
    #[serde(serialize_with = "crate::utils::date_time::format_datetime")]
    pub updated_at: DateTime<Local>,
}