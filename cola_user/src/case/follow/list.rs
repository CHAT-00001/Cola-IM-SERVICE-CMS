// cola_user/src/case/follow/list.rs -- USER - case - 关注 - 列表
// 2026/8/2 22:51 Created.

////////

use anyhow::{Context, Result};
use cola_data::cola_fs::rick_check;
use cola_data::cola_user::command::user::add::UserCommand;
use cola_data::cola_user::command::user::update::UpdateUserCommand;
use cola_data::cola_user::info::user::UserInfo;
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [LIST CASE] - 用户关注列表用例
pub struct UserFollowListCase;

impl UserFollowListCase {
    //

    ////////

    /// # 1. [CASE] - 获取关注列表
    /// * `desc` 返回用户资料列表
    pub async fn case_get_list(
        uid: i64,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
        ctx: AppContext,
    ) -> Result<Vec<UserInfo>, anyhow::Error> {
        // 1. Call .. IDs
        let user_ids = ctx
            .user
            .follow
            .get
            .get_he_follow_ids(user_id, limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 CASE]: ❌️ 获取用户IDs失败: {}", e))?;

        info!("[🗣️ CASE] - ✅️ 用户资料保存成功: uid={},", uid);

        // 2. Call .. Infos

        let infos = ctx
            .user
            .profile
            .get
            .batch_get_infos(user_ids)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 CASE]: ❌️ 批量获取用户资料信息失败: {}", e))?;

        info!("[🗣️ CASE] - ✅️ 批量获取用户资料信息成功: uid={},", uid);

        Ok(infos)
    }

    ////////

    /// # 2. [CASE] - 更新资料
    pub async fn case_update_profile(
        user_id: i64, // 目标用户ID
        mut cmd: UpdateUserCommand,
        ctx: AppContext, // 全局上下文
    ) -> Result<UserInfo, anyhow::Error> {
        // 1. 内容风控
        let check_text = format!("{:?} {:?}", cmd.nickname, cmd.signature);
        let visibility = rick_check(check_text).await;

        // 2. 核心数据持久化与计数更新
        let user_info = ctx
            .user
            .profile
            .add
            .update_user(cmd)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 CASE]: ❌️ 修改用户资料失败: {}", e))?;

        info!(
            "[🗣️ CASE] - ✅️ 修改用户资料成功: uid={}, visibility={}",
            user_id, visibility
        );

        Ok(user_info)
    }
}

//////// END
