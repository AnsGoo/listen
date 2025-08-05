use dotenv::dotenv;
use sea_orm::{ ConnectOptions, Database, DatabaseConnection};
use std::{env, time::Duration, sync::Arc};
use crate::models::artist::{ArtistRepository, SeaOrmArtistRepository};
use crate::models::song::{SongRepository, SeaOrmSongRepository};
use crate::models::album::{AlbumRepository, SeaOrmAlbumRepository};
use crate::models::user::{UserRepository, SeaOrmUserRepository};

#[derive(Clone)]
pub struct AppConfig {
    pub db: DatabaseConnection,
    pub jwt_secret: String,
    pub port: String,
    pub artist_repo: Arc<dyn ArtistRepository + Send + Sync>,
    pub song_repo: Arc<dyn SongRepository + Send + Sync>,
    pub album_repo: Arc<dyn AlbumRepository + Send + Sync>,
    pub user_repo: Arc<dyn UserRepository + Send + Sync>,
}

impl AppConfig {
    pub async fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
        println!("Using port: {}", port);
        
        
        
        // 初始化日志系统
        if let Err(e) = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .try_init() {
            eprintln!("Failed to initialize tracing subscriber: {}", e);
        }


        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .sqlx_logging(false) // Disable SQLx log
            .sqlx_logging_level(log::LevelFilter::Debug); // Or set SQLx log level   

        // 创建数据库连接
  let db = Database::connect(opt).await.expect("Failed to connect to database");

        // 创建歌手仓库实例
        let artist_repo = Arc::new(SeaOrmArtistRepository::new(Arc::new(db.clone())));
        // 创建歌曲仓库实例
        let song_repo = Arc::new(SeaOrmSongRepository::new(Arc::new(db.clone())));
        // 创建专辑仓库实例
        let album_repo = Arc::new(SeaOrmAlbumRepository::new(Arc::new(db.clone())));
        // 创建用户仓库实例
        let user_repo = Arc::new(SeaOrmUserRepository::new(Arc::new(db.clone())));

        AppConfig {
            db,
            jwt_secret,
            port,
            artist_repo,
            song_repo,
            album_repo,
            user_repo,
        }
    }
}