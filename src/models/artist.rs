use chrono::{ DateTime, Local, NaiveDate, Utc};
use serde::{Serialize, Deserialize};
use sea_orm::{ActiveModelTrait, ActiveValue,DeriveEntityModel, QueryOrder, QuerySelect};
use sea_orm::entity::prelude::*;
use uuid::Uuid;
use std::sync::Arc;


// 定义歌手表实体
#[derive(Debug, Clone, Serialize, Deserialize, DeriveEntityModel, PartialEq)]
#[sea_orm(table_name = "artist")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub sex: Option<String>,
    pub nationality: Option<String>,
    pub birth_date: Option<Date>,
    pub avatar: Option<String>,
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
    // 如果有相关表，可以在这里定义关联
}



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

// 专辑创建请求
#[derive(Debug, Deserialize)]
pub struct CreateArtistData {
    pub name: String,
    pub nationality: Option<String>,
    pub birth_date: Option<String>,
    pub avatar: Option<String>,
    pub created_by: String,
    pub sex: Option<String>,
}

// 专辑创建请求
#[derive(Debug, Deserialize)]
pub struct ArtistQueryData {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub nationality: Option<String>,
    pub sex: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}


// 定义歌手仓库trait
#[async_trait::async_trait]
pub trait ArtistRepository {
    async fn create(&self, data: &CreateArtistData) -> Result<Artist, DbErr>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Artist>, DbErr>;
    async fn find_all(&self, params: &ArtistQueryData) -> Result<Vec<Artist>, DbErr>;
}

// 重命名为Artist以保持兼容性
pub type Artist = Model;


// SeaORM实现的歌手仓库
pub struct SeaOrmArtistRepository {
    db: Arc<DatabaseConnection>,
}

impl SeaOrmArtistRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self {
            db,
        }
    }
}

#[async_trait::async_trait]
impl ArtistRepository for SeaOrmArtistRepository {
    async fn create(&self, data: &CreateArtistData) -> Result<Artist, DbErr> {
        let artist = ActiveModel {
            name: ActiveValue::Set(data.name.clone()),
            nationality: ActiveValue::Set(data.nationality.clone()),
            birth_date: ActiveValue::Set(data.birth_date.clone().map(|date| NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap_or_default())),
            avatar: ActiveValue::Set(data.avatar.clone()),
            sex: ActiveValue::Set(data.sex.clone()),
            created_by: ActiveValue::Set(data.created_by.clone()),
            updated_by: ActiveValue::Set(data.created_by.clone()),
            ..ActiveModel::new()
        };

        artist.insert(&*self.db).await
    }

    async fn find_all(&self, params: &ArtistQueryData) -> Result<Vec<Artist>, DbErr> {
        let mut query = Entity::find();

        if let Some(id) = params.id {
            query = query.filter(Column::Id.eq(id));
        }
        if let Some(name) = &params.name {
            query = query.filter(Column::Name.contains(name));
        }
        if let Some(nationality) = &params.nationality {
            query = query.filter(Column::Nationality.eq(nationality));
        }
        if let Some(sex) = &params.sex {
            query = query.filter(Column::Sex.eq(sex));
        }

        // 分页处理
        if let Some(page) = params.page {
            let page_size = params.page_size.unwrap_or(10);
            query = query.offset((page - 1) * page_size).limit(page_size);
        }

        query.all(&*self.db).await
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Artist>, DbErr> {
        Entity::find_by_id(id).one(&*self.db).await
    }
}
