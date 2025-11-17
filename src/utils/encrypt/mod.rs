/// 加密工具
/// 提供密码加密、JWT Token、Base64 等加密功能

use bcrypt::{verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Algorithm, Validation};
use jsonwebtoken::errors::ErrorKind;
use rand::Rng;
use base64::{Engine as _, engine::general_purpose};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

use crate::common::exception::{AppError, ErrorCode};

/// JWT 载荷结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtPayload {
    /// 会话UUID
    #[serde(default)]
    pub session_uuid: String,
    /// 过期时间（Unix 时间戳）
    pub exp: u64,
    /// 用户ID（字符串格式）
    pub sub: String,
}

impl JwtPayload {
    /// 创建新的 JWT 载荷
    pub fn new(
        user_id: impl Into<String>,
        session_uuid: impl Into<String>,
        expire_seconds: i64,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            session_uuid: session_uuid.into(),
            exp: (now + expire_seconds as u64),
            sub: user_id.into(),
        }
    }
}

/// 加密工具
pub struct CryptoUtils;

impl CryptoUtils {
    /// 对密码进行 Bcrypt 加密
    pub async fn hash_password(password: &str, salt: Option<&str>) -> Result<String, AppError> {
        let salt_value = if let Some(s) = salt {
            s.to_string()
        } else {
            let mut rng = rand::thread_rng();
            let salt_bytes: [u8; 16] = rng.gen();
            general_purpose::STANDARD.encode(salt_bytes)
        };

        let salted_password = format!("{}{}", password, salt_value);
        let hashed = bcrypt::hash(&salted_password, DEFAULT_COST)
            .map_err(|e| AppError::with_details(
                ErrorCode::BusinessError,
                "密码加密失败",
                e.to_string(),
            ))?;

        Ok(format!("{}:{}", salt_value, hashed))
    }

    /// 验证密码
    /// # Arguments
    /// * `password` - 明文密码
    /// * `hashed_password` - 加密后的密码
    ///   - 支持格式1：salt:hash（自定义加盐）
    ///   - 支持格式2：$2b$...（纯bcrypt哈希）
    ///
    /// # Returns
    /// 返回验证结果
    pub async fn verify_password(password: &str, hashed_password: &str) -> Result<bool, AppError> {
        // 如果包含自定义salt（salt:hash格式）
        if hashed_password.contains(':') {
            let parts: Vec<&str> = hashed_password.split(':').collect();
            if parts.len() == 2 {
                let salt = parts[0];
                let hash = parts[1];
                let salted_password = format!("{}{}", password, salt);
                return verify(&salted_password, hash)
                    .map_err(|e| AppError::with_details(
                        ErrorCode::BusinessError,
                        "密码验证失败",
                        e.to_string(),
                    ));
            }
        }

        // 直接验证bcrypt哈希
        verify(password, hashed_password)
            .map_err(|e| AppError::with_details(
                ErrorCode::BusinessError,
                "密码验证失败",
                e.to_string(),
            ))
    }

    /// 生成 JWT Token
    /// # Arguments
    /// * `payload` - JWT 载荷
    /// * `secret` - 密钥
    ///
    /// # Returns
    /// 返回 JWT Token 字符串
    pub fn generate_jwt(payload: &JwtPayload, secret: &str) -> Result<String, AppError> {
        let header = Header::new(Algorithm::HS256);
        let key = EncodingKey::from_secret(secret.as_bytes());

        encode(&header, payload, &key)
            .map_err(|e| AppError::with_details(
                ErrorCode::TokenInvalid,
                "JWT Token 生成失败",
                e.to_string(),
            ))
    }

    /// 验证 JWT Token
    /// # Arguments
    /// * `token` - JWT Token 字符串
    /// * `secret` - 密钥
    ///
    /// # Returns
    /// 返回 JWT 载荷
    pub fn verify_jwt(token: &str, secret: &str) -> Result<JwtPayload, AppError> {
        let key = DecodingKey::from_secret(secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        decode::<JwtPayload>(token, &key, &validation)
            .map(|data| data.claims)
            .map_err(|e| match e.kind() {
                ErrorKind::ExpiredSignature => AppError::new(ErrorCode::TokenExpired),
                ErrorKind::InvalidToken => AppError::new(ErrorCode::TokenInvalid),
                _ => AppError::with_details(
                    ErrorCode::TokenInvalid,
                    "JWT Token 验证失败",
                    e.to_string(),
                ),
            })
    }

    /// 生成随机盐值
    /// # Returns
    /// 返回 Base64 编码的 16 字节盐值
    pub fn generate_salt() -> String {
        let mut rng = rand::thread_rng();
        let salt_bytes: [u8; 16] = rng.gen();
        general_purpose::STANDARD.encode(salt_bytes)
    }

    /// 生成 SHA256 哈希
    /// # Arguments
    /// * `data` - 要哈希的数据
    ///
    /// # Returns
    /// 返回哈希值的十六进制字符串
    pub fn sha256(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Base64 编码
    /// # Arguments
    /// * `data` - 要编码的数据
    ///
    /// # Returns
    /// 返回 Base64 编码字符串
    pub fn base64_encode(data: &str) -> String {
        general_purpose::STANDARD.encode(data)
    }

    /// Base64 解码
    /// # Arguments
    /// * `encoded` - Base64 编码字符串
    ///
    /// # Returns
    /// 返回解码后的字符串
    pub fn base64_decode(encoded: &str) -> Result<String, AppError> {
        general_purpose::STANDARD
            .decode(encoded)
            .map(String::from_utf8)
            .map_err(|e| AppError::with_details(
                ErrorCode::ValidationError,
                "Base64 解码失败",
                e.to_string(),
            ))?
            .map_err(|e| AppError::with_details(
                ErrorCode::ValidationError,
                "UTF-8 转换失败",
                e.to_string(),
            ))
    }

    /// 生成随机字符串
    /// # Arguments
    /// * `length` - 字符串长度
    ///
    /// # Returns
    /// 返回随机字符串
    pub fn random_string(length: usize) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}
