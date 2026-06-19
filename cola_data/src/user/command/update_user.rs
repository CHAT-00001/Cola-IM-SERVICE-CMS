// date/src/user/command/update_user.rs  -- 更新用户资料命令
// 2026/6/18 07:27

////////

use crate::user::entity::user::UserEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserCommand {
    pub nickname: Option<String>,
    pub signature: Option<String>,
    pub avatar: Option<String>,
    pub bg_img: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub birthday: Option<i64>,
    pub sns_url: Option<String>,
}

impl UpdateUserCommand {
    /// 将命令中的变更应用到现有的实体上
    pub fn apply(self, mut entity: UserEntity) -> UserEntity {
        if let Some(v) = self.nickname { entity.user_nickname = Some(v); }
        if let Some(v) = self.signature { entity.signature = Some(v); }
        if let Some(v) = self.avatar { entity.avatar = Some(v); }
        if let Some(v) = self.bg_img { entity.bg_img = Some(v); }
        if let Some(v) = self.email { entity.email = Some(v); }
        if let Some(v) = self.phone { entity.phone = Some(v); }
        if let Some(v) = self.birthday { entity.birthday = Some(v); }
        if let Some(v) = self.sns_url { entity.sns_url = Some(v); }

        entity
    }
}
