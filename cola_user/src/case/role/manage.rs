// user/src/case/role/manage.rs -- USER - case - 角色 - 管理用例
// 2026/8/2 22:49 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::cola_fs::rick_check;
use cola_data::cola_user::command::user::update::UpdateUserCommand;
use cola_data::cola_user::info::role::RoleInfo;
use cola_data::cola_user::info::user::UserInfo;
use port::app::ctx::AppContext;
use service::cola_user::role::manage::RoleManageService;
use tracing::info;

////////

/// # [MANAGE CASE] - 用户角色管理用例
pub struct UserRoleManageCase;

impl UserRoleManageCase {
    //

    ////////

    /// # 1. [CASE] - 启用角色
    /// * `desc`: 管理员启用指定角色
    pub async fn case_enable(
        uid: i64,          // 管理员ID
        role_id: i64,      // 角色ID
        _ctx: &AppContext, // 全局上下文
    ) -> Result<RoleInfo, anyhow::Error> {
        // Call ..
        let info = RoleManageService::enable(uid, role_id)
            .await
            .map_err(|e| anyhow!("[🤐 ROLE MANAGE CASE]: ❌️ 启用角色失败: {}", e))?;
        info!(
            "[🗣️ ROLE MANAGE CASE]: ✅️ 启用角色成功: uid={}, role_id={}",
            uid, role_id
        );
        Ok(info)
    }

    /// # 2. [CASE] - 禁用角色
    /// * `desc`: 管理员禁用指定角色
    pub async fn case_disable(
        uid: i64,          // 管理员ID
        role_id: i64,      // 角色ID
        _ctx: &AppContext, // 全局上下文
    ) -> Result<RoleInfo, anyhow::Error> {
        // Call ..
        let info = RoleManageService::disable(uid, role_id)
            .await
            .map_err(|e| anyhow!("[🤐 ROLE MANAGE CASE]: ❌️ 禁用角色失败: {}", e))?;
        info!(
            "[🗣️ ROLE MANAGE CASE]: ✅️ 禁用角色成功: uid={}, role_id={}",
            uid, role_id
        );
        Ok(info)
    }

    /// # 3. [CASE] - 推荐角色
    /// * `desc`: 管理员推荐角色至首页
    pub async fn case_recommend(
        uid: i64,          // 管理员ID
        role_id: i64,      // 角色ID
        _ctx: &AppContext, // 全局上下文
    ) -> Result<RoleInfo, anyhow::Error> {
        // Call ..
        let info = RoleManageService::recommend(uid, role_id)
            .await
            .map_err(|e| anyhow!("[🤐 ROLE MANAGE CASE]: ❌️ 推荐角色失败: {}", e))?;
        info!(
            "[🗣️ ROLE MANAGE CASE]: ✅️ 推荐角色成功: uid={}, role_id={}",
            uid, role_id
        );
        Ok(info)
    }

    /// # 4. [CASE] - 取消推荐
    /// * `desc`: 管理员取消角色推荐
    pub async fn case_unrecommend(
        uid: i64,          // 管理员ID
        role_id: i64,      // 角色ID
        _ctx: &AppContext, // 全局上下文
    ) -> Result<RoleInfo, anyhow::Error> {
        // Call ..
        let info = RoleManageService::unrecommend(uid, role_id)
            .await
            .map_err(|e| anyhow!("[🤐 ROLE MANAGE CASE]: ❌️ 取消推荐失败: {}", e))?;
        info!(
            "[🗣️ ROLE MANAGE CASE]: ✅️ 取消推荐成功: uid={}, role_id={}",
            uid, role_id
        );
        Ok(info)
    }

    /// # 5. [CASE] - 上架角色
    /// * `desc`: 管理员上架角色
    pub async fn case_list_on(
        uid: i64,          // 管理员ID
        role_id: i64,      // 角色ID
        _ctx: &AppContext, // 全局上下文
    ) -> Result<RoleInfo, anyhow::Error> {
        // Call ..
        let info = RoleManageService::list_on(uid, role_id)
            .await
            .map_err(|e| anyhow!("[🤐 ROLE MANAGE CASE]: ❌️ 上架角色失败: {}", e))?;
        info!(
            "[🗣️ ROLE MANAGE CASE]: ✅️ 上架角色成功: uid={}, role_id={}",
            uid, role_id
        );
        Ok(info)
    }

    /// # 6. [CASE] - 下架角色
    /// * `desc`: 管理员下架角色
    pub async fn case_unlist(
        uid: i64,          // 管理员ID
        role_id: i64,      // 角色ID
        _ctx: &AppContext, // 全局上下文
    ) -> Result<RoleInfo, anyhow::Error> {
        // Call ..
        let info = RoleManageService::unlist(uid, role_id)
            .await
            .map_err(|e| anyhow!("[🤐 ROLE MANAGE CASE]: ❌️ 下架角色失败: {}", e))?;
        info!(
            "[🗣️ ROLE MANAGE CASE]: ✅️ 下架角色成功: uid={}, role_id={}",
            uid, role_id
        );
        Ok(info)
    }

    /// # 7. [CASE] - 备用接口1
    pub async fn case_reserve1(
        uid: i64,          // 管理员ID
        role_id: i64,      // 角色ID
        _ctx: &AppContext, // 全局上下文
    ) -> Result<RoleInfo, anyhow::Error> {
        Ok(RoleInfo {
            id: role_id,
            uid,
            ..Default::default()
        })
    }

    /// # 8. [CASE] - 备用接口2
    pub async fn case_reserve2(
        uid: i64,          // 管理员ID
        role_id: i64,      // 角色ID
        _ctx: &AppContext, // 全局上下文
    ) -> Result<RoleInfo, anyhow::Error> {
        Ok(RoleInfo {
            id: role_id,
            uid,
            ..Default::default()
        })
    }

    ////////

    /// # 9. [CASE] - 更新用户资料
    pub async fn case_update_profile(
        user_id: i64, // 目标用户ID
        mut cmd: UpdateUserCommand,
        ctx: AppContext,
    ) -> Result<UserInfo, anyhow::Error> {
        // 1. 内容风控（标题 + 简介 联合过滤）
        let check_text = format!("{:?} {:?}", cmd.nickname, cmd.signature);

        // 2. Call .. 内容风控
        let visibility = rick_check(check_text).await;

        // 3. 核心数据持久化与计数更新
        let user_info = ctx
            .user
            .profile
            .add
            .update_user(cmd)
            .await
            .map_err(|e| anyhow::anyhow!("CASE: 修改用户资料失败: {}", e))?;

        info!(
            "[🗣️ CASE] - ✅️ 修改用户资料成功: uid={}, visibility={}",
            user_id, visibility
        );

        Ok(user_info)
    }

    ////////
}

//////// END
