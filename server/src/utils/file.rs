
use md5::{Md5, Digest};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
pub fn get_file_hash_code(file_path: &Path)-> Result<String, io::Error>{
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    // 读取文件内容到buffer
    file.read_to_end(&mut buffer)?;
    
    // 创建Md5对象
    let mut hasher = Md5::new();
    // 输入数据到Md5对象
    hasher.update(&buffer);
    // 获取MD5结果并转化为字符串
    let result = hasher.finalize();
    let md5_hash = format!("{:x}", result);
    Ok(md5_hash)
}
