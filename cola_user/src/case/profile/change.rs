// cola_user/src/case/user/manage.rs
// core - USER - case - profile - 修改 用例
// 2026/8/2 22:54 Created.

////////

use anyhow::{Context, Result};
use cola_data::app::ctx::AppContext;
use cola_data::fs::rick_check;
use cola_data::user::command::new::UserCommand;
use cola_data::user::info::user::UserInfo;
use tracing::info;

////////

/// # [CHANGE CASE] - 🚧 修改
/// - `desc`: `用户 资料 修改 用例`
pub struct UserProfileChangeCase;

impl UserProfileChangeCase {
    //

    ////////

    /// # 1. [CASE] - 昵称
    /// * `desc`: `修改用户资料昵称`
    pub async fn case_change_nickname(
        uid: i64,
        cmd: UserCommand,
        ctx: AppContext,
    ) -> Result<UserInfo, anyhow::Error> {
        // 1. 内容风控（标题 + 简介 联合过滤）
        let check_text = format!("{:?} {:?}", cmd.nickname, cmd.signature);

        // ✅ 核心修复：rick_check 异步执行后出来就是 i16，直接 await 拿值，删掉多余的 map_err!?
        let visibility = rick_check(check_text).await;

        // 2. 核心数据持久化与计数更新 (💡 提示：建议让这个 Service 函数返回刚插入成功的 VideoInfo)
        let user_info = ctx
            .user
            .add
            .save_user(cmd)
            .await
            .map_err(|e| anyhow::anyhow!("[CASE]: ❌️ 用户资料保存失败: {}", e))?;

        info!("[CASE] - ✅️ 用户资料保存成功: uid={},", uid);

        Ok(user_info)
    }

    ////////

    /// # 2. [CASE] - 头像
    /// * `desc`: `修改用户资料头像`
    pub async fn case_update_avatar(
        user_id: i64, // 目标用户ID
        mut cmd: UpdateUserCommand,
        ctx: AppContext,
    ) -> Result<UserInfo, anyhow::Error> {
        // 1. 内容风控（标题 + 简介 联合过滤）
        let check_text = format!("{:?} {:?}", cmd.nickname, cmd.signature);

        // ✅ 核心修复：同上，直接接住 i16
        let visibility = rick_check(check_text).await;

        // 2. 核心数据持久化与计数更新
        let user_info = ctx
            .user
            .add
            .update_user(cmd)
            .await
            .map_err(|e| anyhow::anyhow!("[CASE]: ❌️ 修改用户头像失败: {}", e))?;

        info!(
            "[CASE] - ✅️ 修改用户头像成功: uid={}, visibility={}",
            user_id, visibility
        );

        Ok(user_info)
    }

    ////////
}

//////// END
