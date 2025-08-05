use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime,Local};

/// 认证响应视图对象
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponseViewObject {
    /// 用户ID
    pub user_id: Uuid,
    /// 用户名
    pub username: String,
    /// 昵称
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub role: String,
    pub created_by: Option<String>,
    #[serde(serialize_with = "crate::utils::date_time::format_datetime")]
    pub created_at: DateTime<Local>,
    #[serde(serialize_with = "crate::utils::date_time::format_datetime")]
    pub updated_at: DateTime<Local>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]pub struct UserQueryViewObject {
    /// 用户ID
    pub user_id: Option<Uuid>,
    /// 用户名
    pub username: Option<String>,
    /// 昵称
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}