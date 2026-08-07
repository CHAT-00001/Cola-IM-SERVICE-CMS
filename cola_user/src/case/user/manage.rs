// cola_user/src/case/user/manage.rs
// 用户 - case - 用户 - 管理用例
// 2026/8/2 22:48 Created.

////////

use anyhow::Result;
use cola_data::app::ctx::AppContext;
use cola_data::fs::rick_check;
use cola_data::user::command::new::UserCommand;
use cola_data::user::command::user::update::UpdateUserCommand;
use cola_data::user::info::user::UserInfo;
use tracing::info;
////////

pub struct UserManageCase;

impl UserManageCase {
    //

    ////////

    /// # 1. [CASE] - 添加用户
    /// * `desc` 返回用户资料
    pub async fn case_add_new(
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
            .map_err(|e| anyhow::anyhow!("[🤐 CASE]: ❌️ 用户资料保存失败: {}", e))?;

        info!("[🗣️ CASE] - ✅️ 用户资料保存成功: uid={},", uid);

        Ok(user_info)
    }

    ////////

    /// # 2. [CASE] - 更新用户资料
    pub async fn case_update_profile(
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
            .map_err(|e| anyhow::anyhow!("[🤐 CASE]: ❌️ 修改用户资料失败: {}", e))?;

        info!(
            "[🗣️ CASE] - ✅️ 修改用户资料成功: uid={}, visibility={}",
            user_id, visibility
        );

        Ok(user_info)
    }

    ////////

    /// # 3. [CASE] - 管理员修改用户权限
    /// * `desc`: 管理员修改用户权限等级
    pub async fn case_change_permission(
        uid: i64, // 管理员ID
        cmd: UpdateUserCommand, // 权限命令
        ctx: &AppContext, // 全局上下文
    ) -> Result<UserInfo, anyhow::Error> {
        let user_info = ctx
            .user
            .user
            .manage
            .change_permission(uid, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 CASE]: ❌️ 修改用户权限失败: {}", e))?;

        info!("[🗣️ CASE] - ✅️ 修改用户权限成功: uid={}", uid);
        Ok(user_info)
    }

    ////////

    /// # 4. [CASE] - 管理员修改用户状态(下架/冻结/封禁)
    /// * `desc`: 管理员修改用户状态码(0=正常, 1=下架, 2=冻结, 3=封禁)
    pub async fn case_change_state(
        uid: i64, // 管理员ID
        cmd: UpdateUserCommand, // 状态命令
        ctx: &AppContext, // 全局上下文
    ) -> Result<UserInfo, anyhow::Error> {
        let user_info = ctx
            .user
            .user
            .manage
            .change_state(uid, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 CASE]: ❌️ 修改用户状态失败: {}", e))?;

        info!("[🗣️ CASE] - ✅️ 修改用户状态成功: uid={}", uid);
        Ok(user_info)
    }

    ////////

    /// # 5. [CASE] - 管理员修改用户角色
    /// * `desc`: 管理员修改用户角色(普通用户/认证用户/创作者/管理员)
    pub async fn case_change_role(
        uid: i64, // 管理员ID
        cmd: UpdateUserCommand, // 角色命令
        ctx: &AppContext, // 全局上下文
    ) -> Result<UserInfo, anyhow::Error> {
        let user_info = ctx
            .user
            .user
            .manage
            .change_role(uid, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 CASE]: ❌️ 修改用户角色失败: {}", e))?;

        info!("[🗣️ CASE] - ✅️ 修改用户角色成功: uid={}", uid);
        Ok(user_info)
    }

    ////////

    /// # 6. [CASE] - 管理员修改用户类型
    /// * `desc`: 管理员修改用户类型(个人/企业/机构)
    pub async fn case_change_type(
        uid: i64, // 管理员ID
        cmd: UpdateUserCommand, // 类型命令
        ctx: &AppContext, // 全局上下文
    ) -> Result<UserInfo, anyhow::Error> {
        let user_info = ctx
            .user
            .user
            .manage
            .change_type(uid, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 CASE]: ❌️ 修改用户类型失败: {}", e))?;

        info!("[🗣️ CASE] - ✅️ 修改用户类型成功: uid={}", uid);
        Ok(user_info)
    }
}

//////// END
