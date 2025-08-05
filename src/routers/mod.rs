use actix_web::{web};
use super::handlers;
use crate::middlewares::{auth::AuthMiddleware, logger::RequestLogger};


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
             .wrap(RequestLogger)
            // 用户认证路由(不需要中间件)
            .service(web::resource("/auth/register").route(web::post().to(handlers::auth::register)))
            .service(web::resource("/auth/login").route(web::post().to(handlers::auth::login)))
            // 其他需要认证的路由
            .service(
                web::scope("")
                    .wrap(AuthMiddleware)
                    // 歌手管理路由
                    .service(web::resource("/artists")
                        .route(web::get().to(handlers::artists::get_artists))
                        .route(web::post().to(handlers::artists::create_artist))
                    )
                    .service(web::resource("/artists/{id}").route(web::get().to(handlers::artists::get_artist_by_id)))
                    // 专辑管理路由
                    .service(web::resource("/albums")
                        .route(web::get().to(handlers::albums::get_albums))
                        .route(web::post().to(handlers::albums::create_album))
                    )
                    .service(web::resource("/albums/{id}").route(web::get().to(handlers::albums::get_album_by_id)))
                    // 歌曲管理路由
                    .service(web::resource("/songs")
                    .route(web::get().to(handlers::songs::get_songs))
                    .route(web::post().to(handlers::songs::create_song))
                    )
                    .service(web::resource("/songs/{id}").route(web::get().to(handlers::songs::get_song_by_id)))
                    .service(web::resource("/users").route(web::get().to(handlers::users::get_users)))
            )
    );
}