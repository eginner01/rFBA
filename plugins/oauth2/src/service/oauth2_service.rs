//! OAuth2服务层

use sea_orm::*;
use oauth2::{
    AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
    basic::BasicClient,
    reqwest::async_http_client,
};
use serde_json::Value;

use crate::entity::oauth_bind;
use crate::dto::*;
use crate::error::OAuth2Error;

/// OAuth2配置
#[derive(Clone)]
pub struct OAuth2Config {
    pub github_client_id: String,
    pub github_client_secret: String,
    pub github_redirect_uri: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    pub linux_do_client_id: String,
    pub linux_do_client_secret: String,
    pub linux_do_redirect_uri: String,
}

impl Default for OAuth2Config {
    fn default() -> Self {
        Self {
            github_client_id: std::env::var("OAUTH2_GITHUB_CLIENT_ID").unwrap_or_default(),
            github_client_secret: std::env::var("OAUTH2_GITHUB_CLIENT_SECRET").unwrap_or_default(),
            github_redirect_uri: std::env::var("OAUTH2_GITHUB_REDIRECT_URI")
                .unwrap_or_else(|_| "http://localhost:8000/api/v1/oauth2/github/callback".to_string()),
            google_client_id: std::env::var("OAUTH2_GOOGLE_CLIENT_ID").unwrap_or_default(),
            google_client_secret: std::env::var("OAUTH2_GOOGLE_CLIENT_SECRET").unwrap_or_default(),
            google_redirect_uri: std::env::var("OAUTH2_GOOGLE_REDIRECT_URI")
                .unwrap_or_else(|_| "http://localhost:8000/api/v1/oauth2/google/callback".to_string()),
            linux_do_client_id: std::env::var("OAUTH2_LINUX_DO_CLIENT_ID").unwrap_or_default(),
            linux_do_client_secret: std::env::var("OAUTH2_LINUX_DO_CLIENT_SECRET").unwrap_or_default(),
            linux_do_redirect_uri: std::env::var("OAUTH2_LINUX_DO_REDIRECT_URI")
                .unwrap_or_else(|_| "http://localhost:8000/api/v1/oauth2/linux-do/callback".to_string()),
        }
    }
}

/// OAuth2服务
pub struct OAuth2Service;

impl OAuth2Service {
    /// 获取GitHub授权URL
    pub fn get_github_auth_url(config: &OAuth2Config) -> Result<String, OAuth2Error> {
        let client = BasicClient::new(
            ClientId::new(config.github_client_id.clone()),
            Some(ClientSecret::new(config.github_client_secret.clone())),
            AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?,
            Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?),
        )
        .set_redirect_uri(
            RedirectUrl::new(config.github_redirect_uri.clone())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?,
        );

        let (auth_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("user:email".to_string()))
            .url();

        Ok(auth_url.to_string())
    }

    /// 获取Google授权URL
    pub fn get_google_auth_url(config: &OAuth2Config) -> Result<String, OAuth2Error> {
        let client = BasicClient::new(
            ClientId::new(config.google_client_id.clone()),
            Some(ClientSecret::new(config.google_client_secret.clone())),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?,
            Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?),
        )
        .set_redirect_uri(
            RedirectUrl::new(config.google_redirect_uri.clone())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?,
        );

        let (auth_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url();

        Ok(auth_url.to_string())
    }

    /// GitHub回调处理
    pub async fn github_callback(
        config: &OAuth2Config,
        code: String,
    ) -> Result<OAuthCallbackResponse, OAuth2Error> {
        let client = BasicClient::new(
            ClientId::new(config.github_client_id.clone()),
            Some(ClientSecret::new(config.github_client_secret.clone())),
            AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?,
            Some(TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?),
        )
        .set_redirect_uri(
            RedirectUrl::new(config.github_redirect_uri.clone())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?,
        );

        // 交换授权码获取访问令牌
        let token_result = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|e| OAuth2Error::TokenExchangeError(e.to_string()))?;

        let access_token = token_result.access_token().secret().to_string();

        // 获取用户信息
        let user_info = Self::get_github_user_info(&access_token).await?;

        Ok(OAuthCallbackResponse {
            access_token,
            user_info,
        })
    }

    /// Google回调处理
    pub async fn google_callback(
        config: &OAuth2Config,
        code: String,
    ) -> Result<OAuthCallbackResponse, OAuth2Error> {
        let client = BasicClient::new(
            ClientId::new(config.google_client_id.clone()),
            Some(ClientSecret::new(config.google_client_secret.clone())),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?,
            Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?),
        )
        .set_redirect_uri(
            RedirectUrl::new(config.google_redirect_uri.clone())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?,
        );

        // 交换授权码获取访问令牌
        let token_result = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|e| OAuth2Error::TokenExchangeError(e.to_string()))?;

        let access_token = token_result.access_token().secret().to_string();

        // 获取用户信息
        let user_info = Self::get_google_user_info(&access_token).await?;

        Ok(OAuthCallbackResponse {
            access_token,
            user_info,
        })
    }

    /// 获取GitHub用户信息
    async fn get_github_user_info(access_token: &str) -> Result<OAuthUserInfo, OAuth2Error> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", access_token))
            .header("User-Agent", "FastAPI-Best-Architecture-Rust")
            .send()
            .await
            .map_err(|e| OAuth2Error::ApiError(e.to_string()))?;

        let user_data: Value = response
            .json()
            .await
            .map_err(|e| OAuth2Error::ApiError(e.to_string()))?;

        Ok(OAuthUserInfo {
            provider: "github".to_string(),
            provider_user_id: user_data["id"].as_i64().unwrap_or(0).to_string(),
            username: user_data["login"].as_str().unwrap_or("").to_string(),
            email: user_data["email"].as_str().map(|s| s.to_string()),
            avatar_url: user_data["avatar_url"].as_str().map(|s| s.to_string()),
        })
    }

    /// 获取Google用户信息
    async fn get_google_user_info(access_token: &str) -> Result<OAuthUserInfo, OAuth2Error> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| OAuth2Error::ApiError(e.to_string()))?;

        let user_data: Value = response
            .json()
            .await
            .map_err(|e| OAuth2Error::ApiError(e.to_string()))?;

        Ok(OAuthUserInfo {
            provider: "google".to_string(),
            provider_user_id: user_data["id"].as_str().unwrap_or("").to_string(),
            username: user_data["name"].as_str().unwrap_or("").to_string(),
            email: user_data["email"].as_str().map(|s| s.to_string()),
            avatar_url: user_data["picture"].as_str().map(|s| s.to_string()),
        })
    }

    /// 获取LinuxDo授权URL
    pub fn get_linux_do_auth_url(config: &OAuth2Config) -> Result<String, OAuth2Error> {
        let client = BasicClient::new(
            ClientId::new(config.linux_do_client_id.clone()),
            Some(ClientSecret::new(config.linux_do_client_secret.clone())),
            AuthUrl::new("https://connect.linux.do/oauth2/authorize".to_string())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?,
            Some(TokenUrl::new("https://connect.linux.do/oauth2/token".to_string())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?),
        )
        .set_redirect_uri(
            RedirectUrl::new(config.linux_do_redirect_uri.clone())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?,
        );

        let (auth_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("read".to_string()))
            .url();

        Ok(auth_url.to_string())
    }

    /// LinuxDo回调处理
    pub async fn linux_do_callback(
        config: &OAuth2Config,
        code: String,
    ) -> Result<OAuthCallbackResponse, OAuth2Error> {
        let client = BasicClient::new(
            ClientId::new(config.linux_do_client_id.clone()),
            Some(ClientSecret::new(config.linux_do_client_secret.clone())),
            AuthUrl::new("https://connect.linux.do/oauth2/authorize".to_string())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?,
            Some(TokenUrl::new("https://connect.linux.do/oauth2/token".to_string())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?),
        )
        .set_redirect_uri(
            RedirectUrl::new(config.linux_do_redirect_uri.clone())
                .map_err(|e| OAuth2Error::ConfigError(e.to_string()))?,
        );

        // 交换授权码获取访问令牌
        let token_result = client
            .exchange_code(AuthorizationCode::new(code))
            .request_async(async_http_client)
            .await
            .map_err(|e| OAuth2Error::TokenExchangeError(e.to_string()))?;

        let access_token = token_result.access_token().secret().to_string();

        // 获取用户信息
        let user_info = Self::get_linux_do_user_info(&access_token).await?;

        Ok(OAuthCallbackResponse {
            access_token,
            user_info,
        })
    }

    /// 获取LinuxDo用户信息
    async fn get_linux_do_user_info(access_token: &str) -> Result<OAuthUserInfo, OAuth2Error> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://connect.linux.do/api/user")
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .map_err(|e| OAuth2Error::ApiError(e.to_string()))?;

        let user_data: Value = response
            .json()
            .await
            .map_err(|e| OAuth2Error::ApiError(e.to_string()))?;

        Ok(OAuthUserInfo {
            provider: "linux_do".to_string(),
            provider_user_id: user_data["id"].as_i64().unwrap_or(0).to_string(),
            username: user_data["username"].as_str().unwrap_or("").to_string(),
            email: user_data["email"].as_str().map(|s| s.to_string()),
            avatar_url: user_data["avatar_url"].as_str().map(|s| s.to_string()),
        })
    }

    /// 绑定OAuth账号
    pub async fn bind_oauth(
        db: &DatabaseConnection,
        user_id: i64,
        provider: &str,
        provider_user_id: &str,
        access_token: &str,
    ) -> Result<OAuthBindInfo, OAuth2Error> {
        let now = chrono::Utc::now().naive_utc();
        
        let bind = oauth_bind::ActiveModel {
            user_id: Set(user_id),
            provider: Set(provider.to_string()),
            provider_user_id: Set(provider_user_id.to_string()),
            access_token: Set(Some(access_token.to_string())),
            created_time: Set(now),
            ..Default::default()
        };

        let result = bind
            .insert(db)
            .await
            .map_err(|e| OAuth2Error::DatabaseError(e.to_string()))?;

        Ok(OAuthBindInfo {
            id: result.id,
            user_id: result.user_id,
            provider: result.provider,
            provider_user_id: result.provider_user_id,
            created_time: result.created_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        })
    }

    /// 解绑OAuth账号
    pub async fn unbind_oauth(
        db: &DatabaseConnection,
        user_id: i64,
        provider: &str,
    ) -> Result<(), OAuth2Error> {
        oauth_bind::Entity::find_by_user_and_provider(user_id, provider)
            .one(db)
            .await
            .map_err(|e| OAuth2Error::DatabaseError(e.to_string()))?
            .ok_or(OAuth2Error::NotFound("绑定不存在".to_string()))?
            .delete(db)
            .await
            .map_err(|e| OAuth2Error::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// 检查OAuth绑定
    pub async fn get_bind(
        db: &DatabaseConnection,
        user_id: i64,
        provider: &str,
    ) -> Result<Option<oauth_bind::Model>, OAuth2Error> {
        oauth_bind::Entity::find_by_user_and_provider(user_id, provider)
            .one(db)
            .await
            .map_err(|e| OAuth2Error::DatabaseError(e.to_string()))
    }
}
