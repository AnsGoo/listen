use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, DbErr, DeriveEntityModel, DeriveRelation, EntityTrait, EnumIter, QueryFilter, QueryOrder, QuerySelect};
use uuid::Uuid;
use sea_orm::entity::prelude::*;
use std::sync::Arc;

// 定义歌曲表实体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "song")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub album_id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub genre: Option<String>,
    pub duration: u32,
    pub quality: String,
    pub file_path: String,
    #[sea_orm(indexed)]
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub created_by: String,
    pub updated_by: String,
    #[sea_orm(indexed,default=false)]
    pub delete_flag: bool,
}

// 定义关联
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    // 关联专辑表
    #[sea_orm(belongs_to = "super::album::Entity", from = "Column::AlbumId", to = "super::album::Column::Id")]
    Album,
    // 关联歌手表
    #[sea_orm(belongs_to = "super::artist::Entity", from = "Column::ArtistId", to = "super::artist::Column::Id")]
    Artist,
}

// 为Model实现ActiveModelBehavior
impl sea_orm::ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: ActiveValue::Set(Uuid::now_v7()),
            created_at: ActiveValue::Set(Local::now().into()),
            updated_at: ActiveValue::Set(Local::now().into()),
            delete_flag: ActiveValue::Set(false),
            ..ActiveModelTrait::default()
        }
    }
}

// 定义歌曲仓库 trait
#[async_trait::async_trait]
pub trait SongRepository: Send + Sync {
    async fn create(&self, request: &CreateSongRequest) -> Result<Song, DbErr>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Song>, DbErr>;
    async fn find_all(&self, params: &SongQueryParams) -> Result<Vec<Song>, DbErr>;
}

// SeaORM 实现的歌曲仓库
pub struct SeaOrmSongRepository {
    db: Arc<DatabaseConnection>,
}

impl SeaOrmSongRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self {
            db,
        }
    }
}

#[async_trait::async_trait]
impl SongRepository for SeaOrmSongRepository {
    async fn create(&self, request: &CreateSongRequest) -> Result<Song, DbErr> {
        let song = ActiveModel {
            album_id: ActiveValue::Set(request.album_id),
            artist_id: ActiveValue::Set(request.artist_id),
            title: ActiveValue::Set(request.title.clone()),
            genre: ActiveValue::Set(request.genre.clone()),
            duration: ActiveValue::Set(request.duration),
            quality: ActiveValue::Set(request.quality.clone()),
            file_path: ActiveValue::Set(request.file_path.clone()),
            ..ActiveModel::new()
        };

        song.insert(&*self.db).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Song>, DbErr> {
        Entity::find_by_id(id).one(&*self.db).await
    }

    async fn find_all(&self, params: &SongQueryParams) -> Result<Vec<Song>, DbErr> {
        let mut query = Entity::find().order_by_desc(Column::CreatedAt);

        // 添加筛选条件
        if let Some(album_id) = &params.album_id {
            query = query.filter(Column::AlbumId.eq(*album_id));
        }

        if let Some(artist_id) = &params.artist_id {
            query = query.filter(Column::ArtistId.eq(*artist_id));
        }

        if let Some(genre) = &params.genre {
            query = query.filter(Column::Genre.eq(genre));
        }

        if let Some(quality) = &params.quality {
            query = query.filter(Column::Quality.eq(quality));
        }

        // 处理分页
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(20);
        let offset = (page - 1) * page_size;

        query
            .limit(page_size)
            .offset(offset)
            .all(&*self.db)
            .await
    }
}

// 重命名为Song以保持兼容性
pub type Song = Model;

// 歌曲创建请求
#[derive(Debug, Deserialize)]
pub struct CreateSongRequest {
    pub album_id: Uuid,
    pub artist_id: Uuid,
    pub title: String,
    pub genre: Option<String>,
    pub duration: u32,
    pub quality: String,
    pub file_path: String,
}

// 歌曲查询参数
#[derive(Debug, Deserialize)]
pub struct SongQueryParams {
    pub album_id: Option<Uuid>,
    pub artist_id: Option<Uuid>,
    pub genre: Option<String>,
    pub quality: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

// 为Song模型添加数据访问方法
impl Song {
    // 创建新歌曲
    pub async fn create(db: &DatabaseConnection, request: &CreateSongRequest) -> Result<Self, DbErr> {
        let song = ActiveModel {
            album_id: ActiveValue::Set(request.album_id),
            artist_id: ActiveValue::Set(request.artist_id),
            title: ActiveValue::Set(request.title.clone()),
            genre: ActiveValue::Set(request.genre.clone()),
            duration: ActiveValue::Set(request.duration),
            quality: ActiveValue::Set(request.quality.clone()),
            file_path: ActiveValue::Set(request.file_path.clone()),
            ..ActiveModel::new()

        };

        song.insert(db).await
    }

    // 根据ID获取歌曲
    pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<Option<Self>, DbErr> {
        Entity::find()
            .filter(Column::Id.eq(id))
            .one(db)
            .await
    }

    // 获取所有歌曲（支持筛选和分页）
    pub async fn find_all(db: &DatabaseConnection, params: &SongQueryParams) -> Result<Vec<Self>, DbErr> {
        let mut query = Entity::find().order_by_desc(Column::CreatedAt);

        // 添加筛选条件
        if let Some(album_id) = &params.album_id {
            query = query.filter(Column::AlbumId.eq(album_id.to_owned()));
        }

        if let Some(artist_id) = &params.artist_id {
            query = query.filter(Column::ArtistId.eq(artist_id.to_owned()));
        }

        if let Some(genre) = &params.genre {
            query = query.filter(Column::Genre.eq(genre));
        }

        if let Some(quality) = &params.quality {
            query = query.filter(Column::Quality.eq(quality));
        }

        // 处理分页
        let page = params.page.unwrap_or(1);
        let page_size = params.page_size.unwrap_or(20);
        let offset = (page - 1) * page_size;

        query
            .limit(page_size)
            .offset(offset)
            .all(db)
            .await
    }
}