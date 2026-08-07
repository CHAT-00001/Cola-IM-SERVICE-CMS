// cola_user/src/case/follow/list.rs
// 用户 - case - 关注 - 列表
// 2026/8/2 22:51 Created.

////////

use anyhow::{Context, Result};
use cola_data::app::ctx::AppContext;
use cola_data::cola_fs::rick_check;
use cola_data::cola_user::command::new::UserCommand;
use cola_data::cola_user::command::user::update::UpdateUserCommand;
use cola_data::cola_user::info::user::UserInfo;
use tracing::info;

////////

pub struct UserAddCase;

impl UserAddCase {
    //

    ////////

    /// # 1. [CASE] - 我关注的
    /// * `desc` 返回用户资料
    pub async fn case_add_new(
        uid: i64,
        cmd: UserCommand,
        ctx: AppContext,
    ) -> Result<UserInfo, anyhow::Error> {


        // 2. 核心数据持久化与计数更新 (💡 提示：建议让这个 Service 函数返回刚插入成功的 VideoInfo)
        let user_info = ctx
            .user
            .add
            .save_user(cmd)
            .await
            .map_err(|e| anyhow::anyhow!("CASE: 用户资料保存失败: {}", e))?;

        info!("CASE - 用户资料保存成功: uid={},", uid);

        Ok(user_info)
    }

    ////////

    /// # 2. [CASE] - 她关注的
    pub async fn case_update_profile(
        user_id: i64, // 目标用户ID
        mut cmd: UpdateUserCommand,
        ctx: AppContext,  // 全局上下文
    ) -> Result<UserInfo, anyhow::Error> {

        // 1. 内容风控
        let check_text = format!("{:?} {:?}", cmd.nickname, cmd.signature);
        let visibility = rick_check(check_text).await;

        // 2. 核心数据持久化与计数更新
        let user_info = ctx
            .user
            .add
            .update_user(cmd)
            .await
            .map_err(|e| anyhow::anyhow!("CASE: 修改用户资料失败: {}", e))?;

        info!(
            "CASE - 修改用户资料成功: uid={}, visibility={}",
            user_id, visibility
        );

        Ok(user_info)
    }
}

//////// END
