use audiotags::{Tag, TagType};
use audiotags::AudioTag;


/// 解析音乐文件的元数据
pub fn parse_metadata(file_path: &str) -> Result<Box<dyn AudioTag + Send + Sync>, Box<dyn std::error::Error>> {
    // 读取元数据
    let tag = Tag::new().read_from_path(file_path)?;
    Ok(tag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_metadata() {
        let file_path = "./music/阿梨粤-晚风心里吹.mp3";
        // 检查文件是否存在
        if !std::path::Path::new(file_path).exists() {
            eprintln!("测试文件不存在: {}", file_path);
            return;
        }
        let result = parse_metadata(file_path);
        assert!(result.is_ok());
        let tag = result.unwrap();
        println!("Title: {:?}", tag.title());
        println!("Artist: {:?}", tag.artist());
        // println!("Album: {:?}", tag.album());
        println!("Artists: {:?}", tag.artists());
        println!("Duration: {:?}", tag.duration());
        println!("AlbumArtist: {:?}", tag.album_artist());
        println!("Year: {:?}", tag.year());
        println!("Genre: {:?}", tag.genre());
        println!("Track Number: {:?}", tag.track_number());
        println!("Disc Number: {:?}", tag.disc_number());
        println!("Total Discs: {:?}", tag.total_discs());
        println!("Comment: {:?}",  tag.comment());
        println!("Album Title: {:?}", tag.album_title());
        println!("Album Artist: {:?}", tag.album_artist()); 
    }
}