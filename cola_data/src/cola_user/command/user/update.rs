use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::cola_user::entity::user::UserEntity;

/// # [COMMAND] - 用户修改命令
/// * `desc`: 用户修改资料
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserCommand {
    pub nickname: Option<String>,
    pub signature: Option<String>,
    pub avatar: Option<String>,
    pub bg_img: Option<String>,
    pub sns_url: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub birthday: Option<i64>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub updated_at: Option<DateTime<Utc>>,
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
            send_id: "".to_string(),
            user_type: None,
            user_nickname: self.nickname.clone(),
            signature: self.signature.clone(),
            avatar: self.avatar.clone(),
            bg_img: self.bg_img.clone(),
            sns_url: self.sns_url.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            birthday: self.birthday,
            sex: None,
            perm_id: 0,
            likes: None,
            fans: None,
            follows: None,
            level: None,
            author_level: None,
            lat: self.lat,
            lng: self.lng,
            login_ip: "".to_string(),
            register_ip: "".to_string(),
            status: None,
            create_time: 0,
            created_at: None,
            updated_at: self.updated_at,
        }
    }
}