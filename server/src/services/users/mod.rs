pub mod model;
pub use model::{ UserQueryViewObject, UserResponseViewObject};
use std::{fmt, sync::Arc};
use crate::models::{ user::UserRepository,QueryUserData};
#[derive(Debug)]
pub enum UserServiceError {
    DatabaseError(sea_orm::DbErr),
    UserNotFound,
}

impl fmt::Display for UserServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserServiceError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            UserServiceError::UserNotFound => write!(f, "User not found"),
        }
    }
}

pub async fn get_users_service(query: UserQueryViewObject, user_repo: Arc<dyn UserRepository> ) ->Result<Vec<UserResponseViewObject>, UserServiceError>{ 
    let queryParams = QueryUserData {
        user_id: query.user_id,
        username: query.username,
        nickname: query.nickname,
        email: query.email,
        role: query.role,
        page: query.page,
        page_size: query.page_size,
    };
    let users = user_repo.get_users(&queryParams).await.map_err(UserServiceError::DatabaseError)?;
    let user_list =  users.into_iter().map(|user| {
        UserResponseViewObject {
            user_id: user.id,
            username: user.username,
            nickname: user.nickname,
            email: user.email,
            role: user.role,
            created_by: user.created_by,
            created_at: user.created_at,
            updated_at: user.updated_at,
            updated_by: user.updated_by,
        }
    }).collect::<Vec<_>>();
    Ok(user_list)
}