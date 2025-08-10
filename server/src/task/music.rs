use std::fs::File;
use std::io::Write;
use std::path::Path;

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
    pub codec: String,
    pub cover_art: Option<String>,
}

pub fn scan_music() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("./music");
    let cover_dir = "./cover";

    if !path.is_dir() {
        println!("ERROR: Path is not a directory!");
        return Ok(());
    }

    let music_extensions = ["mp3", "flac", "wma", "m4a", "ogg", "aac", "wav"];
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

        let tagged_file = match Probe::open(file_path) {
            Ok(probe) => probe,
            Err(e) => {
                println!("ERROR: Failed to open file {:?}: {}", file_path, e);
                continue;
            }
        };

        let tagged_file = match tagged_file.read() {
            Ok(tf) => tf,
            Err(e) => {
                println!("ERROR: Failed to read metadata from {:?}: {}", file_path, e);
                continue;
            }
        };

        let tag = match tagged_file.primary_tag() {
            Some(primary_tag) => primary_tag,
            None => match tagged_file.first_tag() {
                Some(t) => t,
                None => {
                    println!("WARNING: No tags found in {:?}", file_path);
                    continue;
                }
            }
        };

        let properties = tagged_file.properties();
        let cover = tag.pictures().iter().next();

        let cover_art = match cover {
            Some(cover) => {
                let music_hash_code = get_file_hash_code(file_path)?;
                let mime_type = cover.mime_type().map(|m| m.as_str().to_string()).unwrap_or_else(|| "unknown".to_string());
                let cover_path = format!("{}/{}.{}", cover_dir, music_hash_code, mime_type);
                let mut cover_file = File::create(&cover_path)?;
                cover_file.write_all(cover.data())?;
                Some(cover_path)
            }
            None => None,
        };

        let size = match file_path.metadata() {
            Ok(meta) => meta.len(),
            Err(e) => {
                println!("WARNING: Failed to get metadata for {:?}: {}", file_path, e);
                0
            }
        };

        let path_str = match file_path.to_str() {
            Some(s) => s.to_string(),
            None => {
                println!("WARNING: Non-UTF-8 path: {:?}", file_path);
                continue;
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
            path: path_str,
            size,
            bitrate: properties.audio_bitrate(),
            codec: ext_str.to_string(),
            sample_rate: properties.sample_rate(),
            channels: properties.channels().map(|c| c as u8),
            cover_art,
        };

        println!("{:?}", metadata);
        music_metadata_collection.push(metadata);
    }

    dbg!(music_metadata_collection);
    // 可在此处处理 music_metadata_collection，例如存入数据库等

    Ok(())
}