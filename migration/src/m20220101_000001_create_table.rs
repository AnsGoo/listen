use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create users table
        manager.create_table(
            Table::create()
                .table(User::Table)
                .if_not_exists()
                .col(ColumnDef::new(User::Id).uuid().primary_key())
                .col(ColumnDef::new(User::Username).string().not_null().unique_key())
                .col(ColumnDef::new(User::Nickname).string().null())
                .col(ColumnDef::new(User::Email).string().null())
                .col(ColumnDef::new(User::PasswordHash).string().not_null())
                .col(ColumnDef::new(User::Role).string().not_null().default("user".to_string()))
                .col(ColumnDef::new(User::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                .col(ColumnDef::new(User::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                .col(ColumnDef::new(User::CreatedBy).string().null())
                .col(ColumnDef::new(User::UpdatedBy).string().null())
                .col(ColumnDef::new(User::DeleteFlag).boolean().not_null().default(false))
                .to_owned()
        ).await?;

        // Create artists table
        manager.create_table(
            Table::create()
                .table(Artist::Table)
                .if_not_exists()
                .col(ColumnDef::new(Artist::Id).uuid().primary_key())
                .col(ColumnDef::new(Artist::Name).string().not_null())
                .col(ColumnDef::new(Artist::Sex).string().null())
                .col(ColumnDef::new(Artist::Nationality).string().null())
                .col(ColumnDef::new(Artist::BirthDate).date().null())
                .col(ColumnDef::new(Artist::Avatar).string().null())
                .col(ColumnDef::new(Artist::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                .col(ColumnDef::new(Artist::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                .col(ColumnDef::new(Artist::CreatedBy).string().not_null())
                .col(ColumnDef::new(Artist::UpdatedBy).string().not_null())
                .col(ColumnDef::new(Artist::DeleteFlag).boolean().not_null().default(false))
                .to_owned()
        ).await?;

        // Create albums table
        manager.create_table(
            Table::create()
                .table(Album::Table)
                .if_not_exists()
                .col(ColumnDef::new(Album::Id).uuid().primary_key())
                .col(ColumnDef::new(Album::ArtistId).uuid().not_null())
                .col(ColumnDef::new(Album::Name).string().not_null())
                .col(ColumnDef::new(Album::Description).string().null())
                .col(ColumnDef::new(Album::CoverImage).string().null())
                .col(ColumnDef::new(Album::Genre).string().null())
                .col(ColumnDef::new(Album::ReleaseDate).date().not_null())
                .col(ColumnDef::new(Album::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                .col(ColumnDef::new(Album::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                .col(ColumnDef::new(Album::CreatedBy).string().not_null())
                .col(ColumnDef::new(Album::UpdatedBy).string().not_null())
                .col(ColumnDef::new(Album::DeleteFlag).boolean().not_null().default(false))
                .to_owned()
        ).await?;

        // Create songs table
        manager.create_table(
            Table::create()
                .table(Song::Table)
                .if_not_exists()
                .col(ColumnDef::new(Song::Id).uuid().primary_key())
                .col(ColumnDef::new(Song::AlbumId).uuid().not_null())
                .col(ColumnDef::new(Song::ArtistId).uuid().not_null())
                .col(ColumnDef::new(Song::Title).string().not_null())
                .col(ColumnDef::new(Song::Genre).string().null())
                .col(ColumnDef::new(Song::Duration).integer().not_null())
                .col(ColumnDef::new(Song::Quality).string().not_null())
                .col(ColumnDef::new(Song::FilePath).string().not_null())
                .col(ColumnDef::new(Song::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                .col(ColumnDef::new(Song::UpdatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                .col(ColumnDef::new(Song::CreatedBy).string().not_null())
                .col(ColumnDef::new(Song::UpdatedBy).string().not_null())
                .col(ColumnDef::new(Song::DeleteFlag).boolean().not_null().default(false))
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop songs table
        manager.drop_table(Table::drop().table(Song::Table).to_owned()).await?;

        // Drop albums table
        manager.drop_table(Table::drop().table(Album::Table).to_owned()).await?;

        // Drop artists table
        manager.drop_table(Table::drop().table(Artist::Table).to_owned()).await?;

        // Drop users table
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Nickname,
    Email,
    PasswordHash,
    Role,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
    DeleteFlag,
}

#[derive(DeriveIden)]
enum Artist {
    Table,
    Id,
    Name,
    Sex,
    Nationality,
    BirthDate,
    Avatar,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
    DeleteFlag,
}

#[derive(DeriveIden)]
enum Album {
    Table,
    Id,
    ArtistId,
    Name,
    Description,
    CoverImage,
    Genre,
    ReleaseDate,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
    DeleteFlag,
}

#[derive(DeriveIden)]
enum Song {
    Table,
    Id,
    AlbumId,
    ArtistId,
    Title,
    Genre,
    Duration,
    Quality,
    FilePath,
    CreatedAt,
    UpdatedAt,
    CreatedBy,
    UpdatedBy,
    DeleteFlag,
}
