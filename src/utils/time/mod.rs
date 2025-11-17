/// 时间处理工具
/// 提供各种时间处理和格式化功能

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// 时间戳类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeType {
    /// 秒
    Seconds,
    /// 毫秒
    Milliseconds,
    /// 微秒
    Microseconds,
    /// 纳秒
    Nanoseconds,
}

/// 格式化类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FormatType {
    /// 标准格式: Y-m-d H:i:s
    Standard,
    /// 日期格式: Y-m-d
    Date,
    /// 时间格式: H:i:s
    Time,
    /// 完整格式: Y-m-d H:i:s.v
    Full,
    /// ISO 8601 格式
    ISO8601,
    /// 自定义格式
    Custom(&'static str),
}

/// 时间工具
pub struct TimeUtils;

impl TimeUtils {
    /// 获取当前时间（UTC）
    pub fn now() -> DateTime<Utc> {
        Utc::now()
    }

    /// 获取当前时间戳（秒）
    pub fn timestamp() -> i64 {
        Utc::now().timestamp()
    }

    /// 获取当前时间戳（毫秒）
    pub fn timestamp_millis() -> i64 {
        Utc::now().timestamp_millis()
    }

    /// 时间戳转 DateTime
    pub fn from_timestamp(timestamp: i64) -> DateTime<Utc> {
        Utc.timestamp_opt(timestamp, 0).unwrap()
    }

    /// 时间戳转 DateTime（毫秒）
    pub fn from_timestamp_millis(timestamp: i64) -> DateTime<Utc> {
        Utc.timestamp_millis_opt(timestamp).unwrap()
    }

    /// 格式化时间
    pub fn format_datetime(dt: &DateTime<Utc>, format: FormatType) -> String {
        match format {
            FormatType::Standard => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
            FormatType::Date => dt.format("%Y-%m-%d").to_string(),
            FormatType::Time => dt.format("%H:%M:%S").to_string(),
            FormatType::Full => dt.format("%Y-%m-%d %H:%M:%S.%f").to_string(),
            FormatType::ISO8601 => dt.to_rfc3339(),
            FormatType::Custom(format) => dt.format(format).to_string(),
        }
    }

    /// 解析时间字符串
    pub fn parse(s: &str, format: FormatType) -> Result<DateTime<Utc>, chrono::ParseError> {
        match format {
            FormatType::Standard => DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                .map(|dt| dt.with_timezone(&Utc)),
            FormatType::Date => DateTime::parse_from_str(s, "%Y-%m-%d")
                .map(|dt| dt.with_timezone(&Utc)),
            FormatType::Time => {
                // 时间格式需要添加日期
                let s = format!("1970-01-01 {}", s);
                DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                    .map(|dt| dt.with_timezone(&Utc))
            }
            FormatType::Full => DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S.%f")
                .map(|dt| dt.with_timezone(&Utc)),
            FormatType::ISO8601 => DateTime::parse_from_rfc3339(s)
                .map(|dt| dt.with_timezone(&Utc)),
            FormatType::Custom(format) => DateTime::parse_from_str(s, format)
                .map(|dt| dt.with_timezone(&Utc)),
        }
    }

    /// 获取时间差（秒）
    pub fn diff_seconds(a: &DateTime<Utc>, b: &DateTime<Utc>) -> i64 {
        a.timestamp() - b.timestamp()
    }

    /// 获取时间差（分钟）
    pub fn diff_minutes(a: &DateTime<Utc>, b: &DateTime<Utc>) -> i64 {
        Self::diff_seconds(a, b) / 60
    }

    /// 获取时间差（小时）
    pub fn diff_hours(a: &DateTime<Utc>, b: &DateTime<Utc>) -> i64 {
        Self::diff_minutes(a, b) / 60
    }

    /// 获取时间差（天）
    pub fn diff_days(a: &DateTime<Utc>, b: &DateTime<Utc>) -> i64 {
        Self::diff_hours(a, b) / 24
    }

    /// 判断是否为今天
    pub fn is_today(dt: &DateTime<Utc>) -> bool {
        let now = Utc::now();
        dt.date_naive() == now.date_naive()
    }

    /// 判断是否为昨天
    pub fn is_yesterday(dt: &DateTime<Utc>) -> bool {
        let now = Utc::now();
        let yesterday = now.date_naive().pred_opt().unwrap();
        dt.date_naive() == yesterday
    }

    /// 格式化相对时间（如：2天前，3小时前）
    pub fn format_relative(dt: &DateTime<Utc>) -> String {
        let now = Utc::now();
        let diff = now.signed_duration_since(*dt);

        if diff.num_seconds() < 60 {
            "刚刚".to_string()
        } else if diff.num_minutes() < 60 {
            format!("{}分钟前", diff.num_minutes())
        } else if diff.num_hours() < 24 {
            format!("{}小时前", diff.num_hours())
        } else if diff.num_days() < 7 {
            format!("{}天前", diff.num_days())
        } else {
            Self::format_datetime(dt, FormatType::Standard)
        }
    }

    /// 格式化为友好的日期时间
    pub fn format_friendly(dt: &DateTime<Utc>) -> String {
        if Self::is_today(dt) {
            // 今天的日期，只显示时间
            Self::format_datetime(dt, FormatType::Time)
        } else if Self::is_yesterday(dt) {
            // 昨天的日期
            format!("昨天 {}", Self::format_datetime(dt, FormatType::Time))
        } else {
            // 其他日期
            Self::format_datetime(dt, FormatType::Standard)
        }
    }
}

/// 时间序列化器
#[derive(Debug, Clone)]
pub struct DateTimeSerde(#[allow(dead_code)] DateTime<Utc>);

impl DateTimeSerde {
    /// 序列化（Unix 时间戳，秒）
    pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(dt.timestamp())
    }

    /// 反序列化
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let timestamp = i64::deserialize(deserializer)?;
        Ok(Utc.timestamp_opt(timestamp, 0).unwrap())
    }
}

/// 毫秒时间序列化器
#[derive(Debug, Clone)]
pub struct DateTimeMillisSerde(#[allow(dead_code)] DateTime<Utc>);

impl DateTimeMillisSerde {
    /// 序列化（Unix 时间戳，毫秒）
    pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(dt.timestamp_millis())
    }

    /// 反序列化
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let timestamp = i64::deserialize(deserializer)?;
        Ok(Utc.timestamp_millis_opt(timestamp).unwrap())
    }
}
