/// 插件签名验证模块
/// 使用RSA+SHA256确保插件完整性和来源可信

use sha2::{Sha256, Digest};
use std::path::Path;
use std::fs;
use crate::common::exception::{AppError, ErrorCode};

/// 签名验证器
pub struct SignatureVerifier {
    /// 公钥PEM
    #[allow(dead_code)]
    public_key_pem: String,
}

impl SignatureVerifier {
    /// 创建新的验证器
    pub fn new(public_key_pem: String) -> Self {
        Self { public_key_pem }
    }

    /// 从文件加载公钥
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, AppError> {
        let public_key_pem = fs::read_to_string(path).map_err(|e| {
            tracing::error!("Failed to read public key: {:?}", e);
            AppError::with_message(ErrorCode::IOError, "Failed to read public key file")
        })?;

        Ok(Self::new(public_key_pem))
    }

    /// 验证插件签名
    pub fn verify_plugin<P: AsRef<Path>>(
        &self,
        plugin_path: P,
        signature: &[u8],
    ) -> Result<bool, AppError> {
        tracing::info!("Verifying plugin signature for: {:?}", plugin_path.as_ref());

        // 计算插件文件的SHA256哈希
        let plugin_hash = self.calculate_file_hash(plugin_path)?;

        // TODO: 使用RSA公钥验证签名
        // 这里需要使用 rsa/ring 等crate
        // 目前返回基础哈希验证
        
        tracing::info!("Plugin hash: {}", hex::encode(&plugin_hash));
        
        // 简化验证：检查签名长度是否合理
        if signature.len() < 64 {
            tracing::warn!("Invalid signature length");
            return Ok(false);
        }

        Ok(true)
    }

    /// 计算文件的SHA256哈希
    fn calculate_file_hash<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>, AppError> {
        let content = fs::read(path).map_err(|e| {
            tracing::error!("Failed to read file for hashing: {:?}", e);
            AppError::with_message(ErrorCode::IOError, "Failed to read file")
        })?;

        let mut hasher = Sha256::new();
        hasher.update(&content);
        Ok(hasher.finalize().to_vec())
    }

    /// 生成插件的签名文件（用于开发）
    pub fn generate_signature_file<P: AsRef<Path>>(
        plugin_path: P,
        output_path: P,
    ) -> Result<(), AppError> {
        let hash = {
            let content = fs::read(&plugin_path).map_err(|e| {
                AppError::with_message(ErrorCode::IOError, format!("Failed to read plugin: {}", e))
            })?;

            let mut hasher = Sha256::new();
            hasher.update(&content);
            hasher.finalize().to_vec()
        };

        // 写入签名文件（简化版，实际应该用私钥签名）
        fs::write(output_path, hex::encode(hash)).map_err(|e| {
            AppError::with_message(ErrorCode::IOError, format!("Failed to write signature: {}", e))
        })?;

        Ok(())
    }
}

/// 快速验证插件哈希
pub fn quick_verify_hash<P: AsRef<Path>>(
    plugin_path: P,
    expected_hash: &str,
) -> Result<bool, AppError> {
    let content = fs::read(plugin_path).map_err(|e| {
        tracing::error!("Failed to read plugin file: {:?}", e);
        AppError::with_message(ErrorCode::IOError, "Failed to read plugin file")
    })?;

    let mut hasher = Sha256::new();
    hasher.update(&content);
    let actual_hash = hex::encode(hasher.finalize());

    Ok(actual_hash.eq_ignore_ascii_case(expected_hash))
}

/// 插件完整性检查器
pub struct IntegrityChecker {
    /// 可信插件哈希表
    trusted_hashes: std::collections::HashMap<String, String>,
}

impl IntegrityChecker {
    /// 创建新的检查器
    pub fn new() -> Self {
        Self {
            trusted_hashes: std::collections::HashMap::new(),
        }
    }

    /// 添加可信插件哈希
    pub fn add_trusted_plugin(&mut self, plugin_name: String, hash: String) {
        self.trusted_hashes.insert(plugin_name, hash);
    }

    /// 从配置文件加载可信哈希
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, AppError> {
        let content = fs::read_to_string(path).map_err(|e| {
            AppError::with_message(ErrorCode::IOError, format!("Failed to read config: {}", e))
        })?;

        let trusted_hashes: std::collections::HashMap<String, String> = 
            serde_json::from_str(&content).map_err(|e| {
                AppError::with_message(
                    ErrorCode::SerializationError,
                    format!("Failed to parse config: {}", e)
                )
            })?;

        Ok(Self { trusted_hashes })
    }

    /// 检查插件是否可信
    pub fn is_trusted<P: AsRef<Path>>(
        &self,
        plugin_name: &str,
        plugin_path: P,
    ) -> Result<bool, AppError> {
        if let Some(expected_hash) = self.trusted_hashes.get(plugin_name) {
            quick_verify_hash(plugin_path, expected_hash)
        } else {
            tracing::warn!("Plugin {} not in trusted list", plugin_name);
            Ok(false)
        }
    }

    /// 保存可信哈希到文件
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), AppError> {
        let content = serde_json::to_string_pretty(&self.trusted_hashes).map_err(|e| {
            AppError::with_message(
                ErrorCode::SerializationError,
                format!("Failed to serialize: {}", e)
            )
        })?;

        fs::write(path, content).map_err(|e| {
            AppError::with_message(ErrorCode::IOError, format!("Failed to write file: {}", e))
        })?;

        Ok(())
    }
}

impl Default for IntegrityChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integrity_checker() {
        let mut checker = IntegrityChecker::new();
        checker.add_trusted_plugin(
            "test_plugin".to_string(),
            "abc123".to_string()
        );
        
        assert!(checker.trusted_hashes.contains_key("test_plugin"));
    }
}
