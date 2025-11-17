/// 验证码相关DTO

use serde::Serialize;

/// 获取验证码响应
#[derive(Debug, Serialize)]
pub struct CaptchaResponse {
    pub uuid: String,
    pub img_type: String,
    pub image: String,
}
