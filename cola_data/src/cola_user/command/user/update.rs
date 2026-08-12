// data/src/user/command/user/update.rs -- 用户资料 - 更新
// 2026/5/14 10:20

////////

use crate::cola_user::entity::user::UserEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

////////

/// # [COMMAND] - 用户修改命令
/// * `desc`: 用户修改资料
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserCommand {
    pub nickname: Option<String>,          // 昵称
    pub signature: Option<String>,         // 个签
    pub avatar: Option<String>,            // 头像
    pub bg_img: Option<String>,            // 背景图
    pub sns_url: Option<String>,           // 社交网站
    pub email: Option<String>,             // 邮箱
    pub phone: Option<String>,             // 电话
    pub birthday: Option<i64>,             // 生日
    pub lat: Option<String>,                  // 纬度
    pub lng: Option<String>,                  // 经度
    pub updated_at: Option<DateTime<Utc>>, // 更新时间
}

impl UpdateUserCommand {
    pub fn new() -> Self {
        Self {
            updated_at: Some(Utc::now()),
            ..Default::default()
        }
    }

    /// 将命令转换为用户实体，需要传入 user_id
    /// 注意：对于 Option 字段，如果为 None 则使用默认值（空字符串或0）
    pub fn to_entity(&self, user_id: i64) -> UserEntity {
        UserEntity {
            id: user_id,
            _id: Option::from("".to_string()),
            user_type: None,
            user_nickname: self.nickname.clone(),
            signature: self.signature.clone(),
            avatar: self.avatar.clone(),
            bg_img: self.bg_img.clone(),
            sns_url: self.sns_url.clone(),
            user_email: self.email.clone(),
            phone: self.phone.clone(),
            birthday: self.birthday,
            sex: None,
            perm_id: 0,
            views: None,
            likes: None,
            fans: None,
            follows: None,
            level: None,
            author_level: None,
            lat: self.lat.clone(),
            lng: self.lng.clone(),
            login_ip: Some("".to_string()),
            register_ip: Some("".to_string()),
            status: None,
            is_deleted: None,
            create_time: 0,
            created_at: None,
            updated_at: self.updated_at,
            deleted_at: None,
        }
    }
}

//////// END