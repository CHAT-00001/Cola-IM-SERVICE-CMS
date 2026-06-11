// cola_auth/src/model/vo/session.rs

use cola_data::auth::info::session::SessionInfo;
use cola_data::user::info::user::UserInfo;
use serde::Serialize;

/// # [VO] - 会话视图对象
#[derive(Debug, Serialize, Clone)]
pub struct SessionVo {
    pub session: SessionInfo,
    pub user_info: UserInfo,
    pub is_new_user: bool,
}

/// # [Response] - API 响应包装
#[derive(Debug, Serialize, Clone)]
pub struct SessionResponse(pub SessionVo);

// 1. 为 SessionResponse 实现 Default
impl Default for SessionResponse {
    fn default() -> Self {
        Self(SessionVo {
            // 这里根据你的实际需求设置默认值
            // 如果这些类型没有实现 Default，你需要手动构造空的实例
            session: SessionInfo::default(),
            user_info: UserInfo::default(),
            is_new_user: false,
        })
    }
}

impl SessionVo {
    pub fn new(session: SessionInfo, user_info: UserInfo, is_new_user: bool) -> Self {
        Self {
            session,
            user_info,
            is_new_user,
        }
    }
}

////////