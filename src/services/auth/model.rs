use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
/// 登录请求视图对象
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginViewObject {
    /// 用户名
    pub username: String,
    /// 密码
    pub password: String,
}

/// 注册请求视图对象
#[derive(Debug, Validate,Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterViewObject {
    /// 用户名
    #[validate(length(min = 5, max = 20, message = "用户名必须介于5-20个字符之间"))]
    pub username: String,
    /// 密码
    #[validate(length(min = 8, max = 20, message = "密码必须介于8-20个字符之间"))]
    pub password: String,
}

/// 认证响应视图对象
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponseViewObject {
    /// 用户ID
    pub user_id: Uuid,
    /// 用户名
    pub username: String,
    /// 昵称
    pub nickname: Option<String>,
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间(秒)
    pub expires_in: i64,
    pub email: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseViewObject {
    /// 用户ID
    pub user_id: Uuid,
    /// 用户名
    pub username: String,
    /// 访问令牌
    pub access_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间(秒)
    pub expires_in: i64,
}