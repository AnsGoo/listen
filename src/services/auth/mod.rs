pub mod model;
use self::model::*;
use crate::{models, AppState};
use  crate::utils::Claims;
use actix_web::web;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use std::fmt;
use chrono::{Utc, Duration};

#[derive(Debug)]
pub enum AuthServiceError {
    DatabaseError(sea_orm::DbErr),
    BcryptError(bcrypt::BcryptError),
    JwtError(jsonwebtoken::errors::Error),
    InvalidCredentials,
    UserAlreadyExists,
    PasswordHashError,
}

impl fmt::Display for AuthServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthServiceError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            AuthServiceError::BcryptError(e) => write!(f, "Bcrypt error: {:?}", e),
            AuthServiceError::JwtError(e) => write!(f, "JWT error: {:?}", e),
            AuthServiceError::InvalidCredentials => write!(f, "Invalid username or password"),
            AuthServiceError::UserAlreadyExists => write!(f, "Username already exists"),
            AuthServiceError::PasswordHashError => write!(f, "Failed to hash password"),
        }
    }
}

/// 用户登录服务
pub async fn login_service(
    data: LoginViewObject,
    state: &web::Data<AppState>,
) -> Result<AuthResponseViewObject, AuthServiceError> {
    // 查询用户
    let user = state.config.user_repo.find_by_username(&data.username)
        .await
        .map_err(AuthServiceError::DatabaseError)?;

    let user = match user {
        Some(user) => user,
        None => return Err(AuthServiceError::InvalidCredentials),
    };

    // 验证密码
    let password_valid = verify(&data.password, &user.password_hash)
        .map_err(AuthServiceError::BcryptError)?;

    if !password_valid {
        return Err(AuthServiceError::InvalidCredentials);
    }

    // 生成JWT令牌
    let expiration = Utc::now() + Duration::seconds(3600); // 1 hour expiration
    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username.clone(),
        exp: expiration.timestamp() as i64,
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_bytes()),
    )
    .map_err(AuthServiceError::JwtError)?;

    Ok(AuthResponseViewObject {
        user_id: user.id,
        username: user.username,
        nickname: user.nickname,
        email: user.email,
        role: Some(user.role),
        access_token: token,
        token_type: "bearer".to_string(),
        expires_in: 3600, // 1 hour in seconds
    })
}

/// 用户注册服务
pub async fn register_service(
    data: RegisterViewObject,
    state: &web::Data<AppState>,
) -> Result<LoginResponseViewObject, AuthServiceError> {
    // 检查用户名是否已存在
    let existing_user = state.config.user_repo.find_by_username(&data.username)
        .await
        .map_err(AuthServiceError::DatabaseError)?;

    if existing_user.is_some() {
        return Err(AuthServiceError::UserAlreadyExists);
    }

    // 密码哈希
    let password_hash = hash(&data.password, DEFAULT_COST)
        .map_err(|_| AuthServiceError::PasswordHashError)?;

    // 创建用户数据对象
    let user_data = models::CreateUserData {
        username: data.username.clone(),
        password_hash,
    };

    // 创建用户
    let user = state.config.user_repo.create(&user_data)
        .await
        .map_err(AuthServiceError::DatabaseError)?;

    // 生成JWT令牌
    let expiration = Utc::now() + Duration::seconds(3600);
    let claims = Claims {
        sub: user.id.to_string(),
        username: user.username.clone(),
        exp: expiration.timestamp(),
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(state.config.jwt_secret.as_bytes()),
    )
    .map_err(AuthServiceError::JwtError)?;

    Ok(LoginResponseViewObject {
        user_id: user.id,
        username: user.username,
        access_token: token,
        token_type: "bearer".to_string(),
        expires_in: 3600,
    })
}