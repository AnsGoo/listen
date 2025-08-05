use actix_web::{HttpResponse, Responder, HttpRequest, HttpMessage};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::models::{User};
use sea_orm::DbErr;

// JWT相关常量
pub const JWT_EXPIRATION_SECONDS: i64 = 86400; // 24小时

// JWT声明结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // 用户ID
    pub username: String,
    pub exp: i64,    // 过期时间
}

// 生成JWT令牌
pub fn generate_jwt(user: &User, secret: &str) -> Result<String, Box<dyn std::error::Error>> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .checked_add(Duration::from_secs(JWT_EXPIRATION_SECONDS as u64))
        .ok_or("Invalid expiration time")?
        .as_secs() as i64;

    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username.clone(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

// 验证JWT令牌
pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, Box<dyn std::error::Error>> {
    let decoded = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(decoded.claims)
}

// 处理数据库错误
pub fn handle_db_error(e: DbErr) -> impl Responder {
    log::error!("Database error: {:?}", e);
    HttpResponse::InternalServerError().json("Database operation failed".to_string())
}

// 验证电子邮件格式
pub fn is_valid_email(email: &str) -> bool {
    let re = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    re.is_match(email)
}

// 验证密码强度
pub fn is_valid_password(password: &str) -> bool {
    password.len() >= 8 &&
    password.chars().any(|c| c.is_uppercase()) &&
    password.chars().any(|c| c.is_lowercase()) &&
    password.chars().any(|c| c.is_ascii_digit())
}

// 从请求中获取用户ID
pub fn get_user_id_from_request(req: &HttpRequest) -> Option<String> {
    req.extensions().get::<String>().cloned()
}