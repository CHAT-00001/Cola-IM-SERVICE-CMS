// cola_user/src/model/vo/user.rs  -- 用户 - VO - 用户展示
// 2026/6/18 09:05

//////

use serde::{Deserialize, Serialize};
use cola_data::user::info::user::UserInfo;

//////

/// # [USER VO] - 用户展示层
/// 从 UserInfo 提纯，只暴露前端需要的信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserVo {
    pub id: i64,                       // 用户ID
    pub nickname: String,              // 昵称
    pub avatar_url: String,            // 头像
    pub bg_img: String,                // 主页背景图
    pub signature: Option<String>,     // 签名
    pub age: Option<i16>,              // 年龄
    pub status: i16,                   // 状态
    pub add_time: i64,                 // 注册时间
    pub is_following: bool,            // 当前登录用户是否关注TA
    pub is_online: bool,               // 是否在线
    pub is_streaming: bool,            // 是否直播中
    pub geo_distance: Option<f64>,     // 与请求者的距离（如果带坐标）
}

//////

impl UserVo {
    /// 从 UserInfo 转换为 UserVo
    pub fn from_info(info: &UserInfo) -> Self {
        Self {
            id: info.id,
            nickname: info.nickname.clone(),
            avatar_url: info.avatar_url.clone(),
            bg_img: info.bg_img.clone(),
            signature: info.signature.clone(),
            age: info.age,
            status: info.status,
            add_time: info.add_time,
            is_following: info.is_following,
            is_online: info.is_online,
            is_streaming: info.is_streaming,
            geo_distance: None,
        }
    }

    /// 批量转换
    pub fn from_info_list(infos: &[UserInfo]) -> Vec<Self> {
        infos.iter().map(Self::from_info).collect()
    }

    /// 设置距离
    pub fn with_distance(mut self, distance: Option<f64>) -> Self {
        self.geo_distance = distance;
        self
    }

    /// 设置关注状态
    pub fn with_following(mut self, is_following: bool) -> Self {
        self.is_following = is_following;
        self
    }
}
