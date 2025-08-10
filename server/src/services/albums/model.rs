use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{ DateTime, Local, NaiveDate };

/// 专辑查询视图对象
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumQueryViewObject {
    /// 专辑ID
    pub id: Option<Uuid>,
    /// 歌手ID
    pub artist_id: Option<Uuid>,
    /// 专辑名称
    pub name: Option<String>,
    /// 发行年份
    pub release_year: Option<i32>,
    /// 页码
    pub page: Option<u32>,
    /// 每页数量
    pub page_size: Option<u32>,
}

/// 创建专辑视图对象
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAlbumViewObject {
    /// 专辑名称
    pub name: String,
    /// 歌手ID
    pub artist_id: Uuid,
    /// 封面图片路径
    pub cover_image: String,
    /// 发行日期
    pub release_date: NaiveDate,
    /// 专辑描述
    pub description: Option<String>,
    pub  genre: Option<String>,
}

/// 专辑详情视图对象
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumDetailViewObject {
    /// 专辑ID
    pub id: Uuid,
    /// 专辑名称
    pub name: String,
    /// 歌手ID
    pub artist_id: Uuid,
    /// 封面图片路径
    pub cover_image: String,
    /// 发行日期
    pub release_date: NaiveDate,
    /// 专辑描述
    pub description: Option<String>,
    /// 创建时间
    #[serde(serialize_with = "crate::utils::date_time::format_datetime")]
    pub created_at: DateTime<Local>,
    #[serde(serialize_with = "crate::utils::date_time::format_datetime")]
    pub updated_at: DateTime<Local>,
}