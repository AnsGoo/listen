// 导出所有模型模块
// 导出实体和相关类型
pub mod user;
pub use user::{User, CreateUserData, QueryUserData};

pub mod artist;
pub use artist::{ ArtistQueryData,CreateArtistData};

pub mod album;
pub use album::{ CreateAlbumData, AlbumQueryData};

pub mod song;
pub use song::{Song, CreateSongRequest, SongQueryParams};