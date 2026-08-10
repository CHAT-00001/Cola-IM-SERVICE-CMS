// cola_user/src/case/profiler/get.rs
// 用户 - case - profile - 获取 用例
// 2026/8/2 22:50 Created.

////////

use anyhow::{Context, Result};
use cola_data::cola_fs::rick_check;
use cola_data::cola_user::command::user::add::UserCommand;
use cola_data::cola_user::command::user::update::UpdateUserCommand;
use cola_data::cola_user::info::user::UserInfo;
use port::ctx::AppContext;
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

    /// # 2. [CASE] - 热门
    pub async fn case_get_hot_list(
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
            .map_err(|e| anyhow::anyhow!("[CASE]: ❌️ 修改用户资料失败: {}", e))?;

        info!(
            "[CASE] - ✅️ 修改用户资料成功: uid={}, visibility={}",
            user_id, visibility
        );

        Ok(user_info)
    }

    ////////

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
