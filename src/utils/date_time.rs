use chrono::{DateTime, FixedOffset, Local, Utc};
use serde::{Deserialize, Deserializer, Serializer};

// 将UTC时间序列化为本地时间字符串 (默认使用东八区)
pub fn utc_to_local<S>(utc: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // 转换为东八区时间
    let local_time = utc.with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
    serializer.serialize_str(&local_time.to_rfc3339())
}

// 将本地时间字符串反序列化为UTC时间 (默认解析东八区时间)
pub fn local_to_utc<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    // 解析东八区时间字符串
    DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(serde::de::Error::custom)
}

// 用于serde的with属性: UTC转本地时间
pub mod utc_to_local {
    use super::*;
    pub fn serialize<S>(utc: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        utc_to_local(utc, serializer)
    }
}

// 用于serde的with属性: 本地时间转UTC
pub mod local_to_utc {
    use super::*;
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        local_to_utc(deserializer)
    }
}

pub fn format_datetime<S>(datetime: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // 转换为东八区时间
   let formatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    serializer.serialize_str(&formatted)
}
