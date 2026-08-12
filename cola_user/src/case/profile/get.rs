// cola_user/src/case/profiler/get.rs
// 用户 - case - profile - 获取 用例
// 2026/8/2 22:50 Created.

////////

use anyhow::{Context, Result};
use cola_data::cola_fs::rick_check;
use cola_data::cola_user::command::user::add::UserCommand;
use cola_data::cola_user::command::user::update::UpdateUserCommand;
use cola_data::cola_user::info::user::UserInfo;
use port::app::ctx::AppContext;
use tracing::info;

////////

pub struct UserProfileGetCase;

impl UserProfileGetCase {
    //

    ////////

    /// # 1. [CASE] - 最新
    /// * `desc` 返回用户资料
    pub async fn case_get_new_list(
        uid: i64,
        limit: i64,
        offset: i64,
        ctx: AppContext,
    ) -> Result<Vec<UserInfo>, anyhow::Error> {

        // 1. 用户信息列表
        let user_info = ctx
            .user
            .user
            .list
            .get_new_list(uid, limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 CASE]: ❌️ 获取最新的用户资料列表失败: {}", e))?;

        info!("[🗣️ CASE] - ✅️ 获取最新的用户资料列表成功: uid={},", uid);

        // 2. 组装 VO

        Ok(user_info)
    }

    ////////


    /// # 4. [CASE] - 搜索
    pub async fn case_get_keyword_list(
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
            .user
            .add
            .update_user(cmd)
            .await
            .map_err(|e| anyhow::anyhow!("[CASE]: ❌️ 搜索用户列表失败: {}", e))?;

        info!(
            "[CASE] - ✅️ 搜索用户列表成功: uid={}, visibility={}",
            user_id, visibility
        );

        Ok(user_info)
    }

    ////////
}

//////// END
