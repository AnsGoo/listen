use std::fs::{create_dir, File};
use std::io::Write;
use std::path::Path;
use lofty::picture::{MimeType, Picture};
use lofty::prelude::*;
use lofty::probe::Probe;
use lofty::tag::ItemKey;
use walkdir::WalkDir;

use crate::utils::get_file_hash_code;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicMetadata {
    pub name: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub album_artist: Option<String>,
    pub duration: u128,
    pub track: Option<u32>,
    pub year: Option<u32>,
    pub path: String,
    pub size: u64,
    pub bitrate: Option<u32>,
    pub sample_rate: Option<u32>,
    pub channels: Option<u8>,
    pub cover_art: Option<String>,
}



enum MusicMetadataError {
    TagNotFound,
    FailedToReadMetadata,
    TagNotFond,
    ProbeFailedFile,
    GenareHashcodeError
}

const COVER_DIR: &str = "../cover";


use std::error::Error;
use std::fs;

use std::path::PathBuf;
use log::error;

pub fn save_cover(hash_code: String, picture: &Picture) -> Result<String, Box<dyn Error>> {
    
    // 验证图片数据不为空
    if picture.data().is_empty() {
        error!("Cannot save empty picture data");
        return Err("Empty picture data".into());
    }
    
    // 创建封面目录（带错误日志）
    let cover_dir = PathBuf::from(COVER_DIR);
    fs::create_dir_all(&cover_dir)
        .map_err(|e| {
            error!("Failed to create cover directory {:?}: {}", cover_dir, e);
            e
        })?;
    
    match picture.mime_type() {
        Some(mime_type) => {
            // 创建封面目录（如果不存在）
            // 从MIME类型提取文件扩展名
            let ext = mime_type.as_str().split('/').last().ok_or("Invalid MIME type")?;
            let filename = format!("{}.{}", hash_code, ext);
            let cover_path = format!("{}/{}", COVER_DIR, filename);
            
            // 创建并写入文件
            let mut cover_file = File::create(&cover_path)
                .map_err(|e| {
                    error!("Failed to create cover file {:?}: {}", cover_path, e);
                    e
                })?;
            
            // 写入图片数据并验证写入大小
           cover_file.write_all(picture.data())
                .map(|_| picture.data().len())
                .map_err(|e| {
                    error!("Failed to write cover data to {:?}: {}", cover_path, e);
                    e
                })?;
            // 记录成功信息
            log::info!("Successfully saved cover image)");
            Ok(filename)
        },
        None => {
            Err("Missing MIME type for cover image".into())
        }
    }
}


pub fn get_music_metadata(file_path: &Path) -> Result<MusicMetadata, MusicMetadataError> {

    let tagged_file = match Probe::open(file_path) {
        Ok(probe) => probe,
        Err(e) => {
            println!("ERROR: Failed to open file {:?}: {}", file_path, e);
            return Err(MusicMetadataError::ProbeFailedFile);
        }
    };

    let tagged_file = match tagged_file.read() {
        Ok(tf) => tf,
        Err(e) => {
            println!("ERROR: Failed to read metadata from {:?}: {}", file_path, e);
            return Err(MusicMetadataError::FailedToReadMetadata);
        }
    };

    let tag = match tagged_file.primary_tag() {
        Some(primary_tag) => primary_tag,
        None => match tagged_file.first_tag() {
            Some(t) => t,
            None => {
                println!("WARNING: No tags found in {:?}", file_path);
                return Err(MusicMetadataError::TagNotFond);
            }
        }
    };

    let properties = tagged_file.properties();

    let mut cover_art: Option<String> = None;
    if let Some(cover) = tag.pictures().iter().next() {
        match get_file_hash_code(file_path) {
            Ok(hash_code) => {
                match save_cover(hash_code.clone(), cover) {
                    Ok(val) => cover_art = Some(val),
                    Err(e) => log::error!("Failed to save cover art for {:?}: {}", file_path, e)
                }
            },
            Err(e) => log::error!("Failed to generate file hash code for {:?}: {}", file_path, e)
        }
    }
    

    let size = match file_path.metadata() {
        Ok(meta) => meta.len(),
        Err(e) => {
            println!("WARNING: Failed to get metadata for {:?}: {}", file_path, e);
            0
        }
    };


    let metadata = MusicMetadata {
        name: tag.title().as_deref().map(|s| s.to_string()),
        artist: tag.artist().as_deref().map(|s| s.to_string()),
        album: tag.album().as_deref().map(|s| s.to_string()),
        genre: tag.genre().as_deref().map(|s| s.to_string()),
        album_artist: tag.get_string(&ItemKey::AlbumArtist).map(|s| s.to_string()),
        year: tag.year(),
        track: tag.track(),
        duration: 0,
        path: file_path.display().to_string(),
        size,
        bitrate: properties.audio_bitrate(),
        sample_rate: properties.sample_rate(),
        channels: properties.channels().map(|c| c as u8),
        cover_art: cover_art,
    };
    Ok(metadata)


}
pub fn scan_music() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("./music");

    let cover_path = Path::new(COVER_DIR);
    if !cover_path.exists() {
        let _ = create_dir(cover_path).is_ok();
        return Ok(());
    };
    if !path.is_dir() {
        println!("ERROR: Path is not a directory!");
        return Ok(());
    }

    let music_extensions = ["mp3", "flac", "m4a", "ogg", "aac", "wav"];
    let mut music_metadata_collection = Vec::new();

    for entry in WalkDir::new(path)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_path = entry.path();
        if !file_path.is_file() {
            continue;
        }

        let ext = match file_path.extension() {
            Some(ext) => ext.to_ascii_lowercase(),
            None => continue,
        };

        let ext_str = match ext.to_str() {
            Some(s) => s,
            None => continue,
        };

        if !music_extensions.contains(&ext_str) {
            continue;
        }

        println!("{:?}", file_path);

       match get_music_metadata(&file_path) {
           Ok(metadata) => {
               println!("{:?}", metadata);
               music_metadata_collection.push(metadata);
           }

           _ => {}
       }


    }
    dbg!(music_metadata_collection);
    // 可在此处处理 music_metadata_collection，例如存入数据库等

    Ok(())
}