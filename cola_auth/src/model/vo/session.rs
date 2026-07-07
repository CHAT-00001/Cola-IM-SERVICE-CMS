// cola_auth/src/model/vo/session.rs

use cola_data::auth::info::session::SessionInfo;
use cola_data::user::info::user::UserInfo;
use serde::Serialize;

/// # [VO] - 登录/注册响应体
/// * `auth`       存储认证令牌信息（access_token, refresh_token, 过期时间）
/// * `user_info`   用户资料
/// * `is_new_user` 是否为新注册用户
#[derive(Debug, Serialize, Clone)]
pub struct SignVo {
    pub auth: SessionInfo,
    pub user_info: UserInfo,
    pub is_new_user: bool,
}

/// # [Response] - API 响应包装
#[derive(Debug, Serialize, Clone)]
pub struct SignResponse(pub SignVo);

impl Default for SignResponse {
    fn default() -> Self {
        Self(SignVo {
            auth: SessionInfo::default(),
            user_info: UserInfo::default(),
            is_new_user: false,
        })
    }
}

impl SignVo {
    pub fn new(auth: SessionInfo, user_info: UserInfo, is_new_user: bool) -> Self {
        Self {
            auth,
            user_info,
            is_new_user,
        }
    }
}

////////
