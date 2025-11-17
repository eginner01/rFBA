//! OAuth2 DTO定义

use serde::{Deserialize, Serialize};
use validator::Validate;

/// OAuth绑定信息
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthBindInfo {
    pub id: i64,
    pub user_id: i64,
    pub provider: String,
    pub provider_user_id: String,
    pub created_time: String,
}

/// 绑定OAuth请求
#[derive(Debug, Deserialize, Validate)]
pub struct BindOAuthParam {
    #[validate(length(min = 1, max = 50))]
    pub provider: String,
    
    #[validate(length(min = 1))]
    pub code: String,
    
    pub redirect_uri: Option<String>,
}

/// 解绑OAuth请求
#[derive(Debug, Deserialize, Validate)]
pub struct UnbindOAuthParam {
    #[validate(length(min = 1, max = 50))]
    pub provider: String,
}

/// OAuth回调响应
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthCallbackResponse {
    pub access_token: String,
    pub user_info: OAuthUserInfo,
}

/// OAuth用户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthUserInfo {
    pub provider: String,
    pub provider_user_id: String,
    pub username: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}

/// API响应
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            msg: "操作成功".to_string(),
            data: Some(data),
        }
    }
    
    pub fn success_msg(msg: &str) -> Self {
        Self {
            code: 200,
            msg: msg.to_string(),
            data: None,
        }
    }
}
