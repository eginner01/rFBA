use crate::app::auth::dto::{LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse, UserInfo};
use crate::common::exception::{AppError, ErrorCode};
use crate::database::{DatabaseConnection, user_repo::UserRepository as UserRepo};
use crate::utils::encrypt::{CryptoUtils, JwtPayload};
use sea_orm::DbErr;
use uuid::Uuid;

const ACCESS_TOKEN_EXPIRE_SECONDS: i64 = 3600 * 24;
const REFRESH_TOKEN_EXPIRE_SECONDS: i64 = 3600 * 24 * 7;

pub struct AuthService {
    jwt_secret: String,
}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

    pub async fn login(
        &self,
        request: &LoginRequest,
        db: &DatabaseConnection,
    ) -> Result<LoginResponse, AppError> {
        let login_account = request
            .select_account
            .as_ref()
            .cloned()
            .unwrap_or_else(|| request.username.clone());

        let user_model = match UserRepo::find_by_username(&login_account, db).await {
            Ok(user) => user,
            Err(DbErr::RecordNotFound(_)) => {
                return Err(AppError::with_message(
                    ErrorCode::AuthenticationFailed,
                    "用户不存在",
                ));
            }
            Err(e) => {
                tracing::error!("Database query failed: account={}, error={}", login_account, e);
                return Err(AppError::with_details(
                    ErrorCode::DatabaseError,
                    "数据库查询失败",
                    e.to_string(),
                ));
            }
        };
        
        // 检查用户是否被禁用
        if user_model.status == 0 {
            return Err(AppError::new(ErrorCode::AuthenticationFailed));
        }

        // 检查用户是否被删除
        if user_model.del_flag == 1 {
            return Err(AppError::new(ErrorCode::AuthenticationFailed));
        }

        // 2. 验证密码
        let password = user_model.password.as_ref().ok_or_else(|| {
            AppError::with_details(
                ErrorCode::AuthenticationFailed,
                "密码验证失败",
                "用户密码不存在".to_string(),
            )
        })?;
        let is_valid = CryptoUtils::verify_password(&request.password, password)
            .await
            .map_err(|e| AppError::with_details(
                ErrorCode::AuthenticationFailed,
                "密码验证失败",
                e.to_string(),
            ))?;

        if !is_valid {
            return Err(AppError::new(ErrorCode::AuthenticationFailed));
        }

        // 3. 生成session UUID
        let session_uuid = Uuid::new_v4().to_string();

        // 4. 生成JWT payload
        let payload = JwtPayload::new(
            user_model.id.to_string(), // sub: 用户ID（字符串）
            session_uuid.clone(),       // session_uuid: 会话UUID
            ACCESS_TOKEN_EXPIRE_SECONDS, // exp: 过期时间
        );

        let access_token = CryptoUtils::generate_jwt(&payload, &self.jwt_secret)?;

        // 5. 计算access token过期时间
        let expire_time = chrono::Utc::now() + chrono::Duration::seconds(ACCESS_TOKEN_EXPIRE_SECONDS);
        let access_token_expire_time = expire_time.naive_local();

        // 6. 构建用户信息
        let user_info = UserInfo {
            id: user_model.id,
            uuid: user_model.uuid,
            username: user_model.username,
            nickname: user_model.nickname,
            email: user_model.email,
            phone: user_model.phone,
            avatar: user_model.avatar,
            dept_id: user_model.dept_id,
            status: user_model.status,
            is_superuser: user_model.is_superuser,
            is_staff: user_model.is_staff,
            is_multi_login: user_model.is_multi_login,
            join_time: user_model.join_time,
            last_login_time: user_model.last_login_time,
            dept: None, // TODO: 从部门表获取部门名称
            roles: vec![], // TODO: 从角色表获取角色名称列表
        };

        Ok(LoginResponse {
            access_token,
            access_token_expire_time,
            session_uuid,
            user: user_info,
        })
    }

    /// 刷新Token - 与Python版本create_new_token逻辑一致
    pub fn refresh_token(
        &self,
        request: &RefreshTokenRequest,
    ) -> Result<RefreshTokenResponse, AppError> {
        // 1. 验证刷新Token - 与Python版本jwt_decode一致
        let jwt_payload = CryptoUtils::verify_jwt(&request.refresh_token, &self.jwt_secret)
            .map_err(|_| AppError::new(ErrorCode::TokenExpired))?;

        // 2. 验证payload包含必要字段 - 与Python版本一致
        if jwt_payload.session_uuid.is_empty() || jwt_payload.sub.is_empty() {
            return Err(AppError::new(ErrorCode::TokenInvalid));
        }

        // 3. 重新生成access_token - 使用相同的session_uuid
        let new_payload = JwtPayload::new(
            jwt_payload.sub.clone(),           // 保持用户ID
            jwt_payload.session_uuid.clone(),  // 使用原有session_uuid
            ACCESS_TOKEN_EXPIRE_SECONDS,
        );

        let access_token = CryptoUtils::generate_jwt(&new_payload, &self.jwt_secret)?;

        // 4. 计算access token过期时间
        let expire_time = chrono::Utc::now() + chrono::Duration::seconds(ACCESS_TOKEN_EXPIRE_SECONDS);
        let access_token_expire_time = expire_time.naive_local();

        Ok(RefreshTokenResponse {
            access_token,
            access_token_expire_time,
            session_uuid: jwt_payload.session_uuid.clone(),
        })
    }

    /// 登出
    pub async fn logout(&self, _token: &str) -> Result<(), AppError> {
        // TODO: 实现Token黑名单机制 - 可选功能
        Ok(())
    }
}
