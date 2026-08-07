// service/src/cola_user/profile/add.rs
// 服务 - 可乐用户 - 资料名片 - 发布
// 2026/8/2 21:53 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::cola_user::command::profile::ProfileCommand;
use cola_data::cola_user::info::profile::ProfileInfo;
use repository::cola_user::pg::profile::add::ProfileAddRepo;

////////

/// # [PROFILE SERVICE] - 发布
/// * `desc`: `🗣 可乐用户 - 用户资料名片发布服务`
pub struct ProfileAddService;

impl ProfileAddService {

    ////////

    /// # 1. [SERVICE] - 创建/更新资料名片
    /// * `desc`: UPSERT 模式，不存在则创建，存在则更新
    pub async fn upsert_profile(cmd: &ProfileCommand) -> Result<ProfileInfo, anyhow::Error> {
        let _id = ProfileAddRepo::pg_upsert_profile(cmd)
            .await
            .map_err(|e| anyhow!("[🤐 PROFILE SERVICE]: ❌️ 保存资料名片失败: {}", e))?;

        let info = ProfileInfo {
            user_id: cmd.user_id,
            nickname: cmd.nickname.clone(),
            avatar: cmd.avatar.clone(),
            bg_img: cmd.bg_img.clone(),
            signature: cmd.signature.clone(),
            birthday: cmd.birthday,
            sex: cmd.sex,
            email: cmd.email.clone(),
            phone: cmd.phone.clone(),
            sns_url: cmd.sns_url.clone(),
            label: cmd.label.clone(),
            add_time: 0,
            upd_time: 0,
        };

        tracing::info!("[🗣️ PROFILE SERVICE]: ✅️ 资料名片保存成功, user_id={}", cmd.user_id);
        Ok(info)
    }

    ////////

    /// # 2. [SERVICE] - 获取资料名片
    /// * `desc`: 根据 user_id 查询资料名片
    pub async fn get_profile(user_id: i64) -> Result<Option<ProfileInfo>, anyhow::Error> {
        let entity = ProfileAddRepo::pg_find_by_user_id(user_id)
            .await
            .map_err(|e| anyhow!("[🤐 PROFILE SERVICE]: ❌️ 查询资料名片失败: {}", e))?;

        match entity {
            Some(e) => {
                let info = ProfileInfo {
                    user_id: e.id,
                    nickname: e.user_nickname.unwrap_or_default(),
                    avatar: e.avatar.unwrap_or_default(),
                    bg_img: e.bg_img.unwrap_or_default(),
                    signature: e.signature.unwrap_or_default(),
                    birthday: e.birthday,
                    sex: e.sex,
                    email: e.email,
                    phone: e.phone,
                    sns_url: e.sns_url,
                    label: None,
                    add_time: e.create_time,
                    upd_time: e.create_time,
                };
                tracing::info!("[🗣️ PROFILE SERVICE]: ✅️ 查询资料名片成功, user_id={}", user_id);
                Ok(Some(info))
            }
            None => {
                tracing::info!("[🗣️ PROFILE SERVICE]: ✅️ 资料名片不存在, user_id={}", user_id);
                Ok(None)
            }
        }
    }
}

//////// END