// cola_user/src/case/profile/add.rs
// core - 可乐用户 - case - 资料名片 用例
// 2026/8/2 22:49 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::cola_user::command::profile::ProfileCommand;
use cola_data::cola_user::info::profile::ProfileInfo;
use port::app::ctx::AppContext;
use service::cola_user::profile::add::ProfileAddService;
use tracing::info;

////////

/// # [PROFILE CASE] - 资料名片 用例
pub struct UserProfileAddCase;

impl UserProfileAddCase {
    //

    ////////

    /// # 1. [CASE] - 创建/更新资料名片
    /// * `desc`: 强社交资料模型，独立的资料名片
    pub async fn case_upsert_profile(
        cmd: ProfileCommand, // 资料名片命令
        _ctx: &AppContext,   // 全局上下文
    ) -> Result<ProfileInfo, anyhow::Error> {
        // 调用 service 层保存资料名片
        let info = ProfileAddService::upsert_profile(&cmd)
            .await
            .map_err(|e| anyhow!("[🤐 PROFILE CASE]: ❌️ 保存资料名片失败: {}", e))?;

        info!(
            "[🗣️ PROFILE CASE]: ✅️ 资料名片保存成功, user_id={}",
            cmd.user_id
        );
        Ok(info)
    }

    ////////

    /// # 2. [CASE] - 获取资料名片
    /// * `desc`: 根据用户ID查询资料名片
    pub async fn case_get_profile(
        user_id: i64,      // 目标用户ID
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Option<ProfileInfo>, anyhow::Error> {
        let info = ProfileAddService::get_profile(user_id)
            .await
            .map_err(|e| anyhow!("[🤐 PROFILE CASE]: ❌️ 获取资料名片失败: {}", e))?;

        info!(
            "[🗣️ PROFILE CASE]: ✅️ 获取资料名片成功, user_id={}",
            user_id
        );
        Ok(info)
    }
}

//////// END
