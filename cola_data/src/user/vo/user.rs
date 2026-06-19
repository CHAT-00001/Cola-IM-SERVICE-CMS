// cola_date/src/user/vo/user.rs  -- 可乐数据中心 - USER - VO - 用户
// 2026/6/18 09:58

////////

use serde::{Deserialize, Serialize};
use crate::user::info::user::UserInfo;

////////

/// # [VO] - 用户视图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserVO {
    pub info: UserInfo,     // 用户基础信息
    pub is_following: bool, // 是否关注
    pub is_online: bool,    // 是否在线
    pub is_streaming: bool, // 是否在直播
}


////////