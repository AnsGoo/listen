use serde::Serialize;

pub mod albums;
pub mod auth;
pub mod artists;
pub mod songs;
pub mod users;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}
// 为ApiResponse实现Display trait
impl<T: Serialize> std::fmt::Display for ApiResponse<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}",  self.message.clone().expect("No message").to_string())
    }
}