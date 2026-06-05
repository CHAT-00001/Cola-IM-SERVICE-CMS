// cola_auth/src/vo/vo/session  -- 会话视图对象
// 2026/5/22 19:34

////////

use cola_data::auth::info::session::AuthSessionInfo;
use cola_data::user::info::user::UserInfo;

////////

/// # [VO] - 会话响应体
pub struct SessionVo {
    pub token: AuthSessionInfo, // Token
    pub info: UserInfo,         // 用户资料
    pub is_new_user: bool,      // 是否新用户
}



/// # [RESPONSE] - 会话响应体
pub struct SessionResponse {
    pub info: SessionVo,
    pub user_info: UserInfo,
    pub is_new_user: bool,
}