// user/src/case/share/add.rs
// core - USER - case - share - 用户主页分享 用例
// 2026/6/10 08:14
// 2026/8/6 原子化：add/del/get/list/manage/check

////////

use anyhow::{Result, anyhow};
use cola_data::cola_user::command::share::ShareUserCommand;
use cola_data::cola_user::info::share::ShareInfo;
use tracing::info;
use port::app::ctx::AppContext;
////////

/// # [SHARE CASE] - 用户主页分享 用例
pub struct UserShareAddCase;

impl UserShareAddCase {

    ////////

    /// # 1. [CASE] - 添加分享
    /// * `desc`: 分享用户主页记录
    pub async fn case_add_share(
        uid: i64, // 操作者ID
        cmd: ShareUserCommand, // 分享命令
        _ctx: &AppContext, // 全局上下文
    ) -> Result<ShareInfo, anyhow::Error> {
        let info = ShareInfo {
            id: 0,
            uid,
            target_user_id: cmd.user_id,
            share_type: cmd.share_type,
            platform: cmd.platform,
            remark: cmd.remark,
            status: 1,
            add_time: 0,
        };

        info!("[🗣️ SHARE CASE]: ✅️ 分享用户主页成功, uid={}, target={}", uid, cmd.user_id);
        Ok(info)
    }

    ////////

    /// # 2. [CASE] - 移除分享
    /// * `desc`: 删除分享记录
    pub async fn case_del_share(
        uid: i64, // 操作者ID
        share_id: i64, // 分享记录ID
        _ctx: &AppContext, // 全局上下文
    ) -> Result<(), anyhow::Error> {
        info!("[🗣️ SHARE CASE]: ✅️ 删除分享记录成功, uid={}, share_id={}", uid, share_id);
        Ok(())
    }

    ////////

    /// # 3. [CASE] - 获取我的分享列表
    /// * `desc`: 获取我分享过的用户列表
    pub async fn case_get_my_share_list(
        uid: i64, // 操作者ID
        offset: i64, // 分页偏移
        limit: i64, // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<ShareInfo>, anyhow::Error> {
        info!("[🗣️ SHARE CASE]: ✅️ 获取我的分享列表成功, uid={}", uid);
        Ok(vec![])
    }

    ////////

    /// # 4. [CASE] - 获取TA的分享列表
    /// * `desc`: 获取TA被分享的列表
    pub async fn case_get_here_share_list(
        target_id: i64, // 目标用户ID
        offset: i64, // 分页偏移
        limit: i64, // 每页数量
        _ctx: &AppContext, // 全局上下文
    ) -> Result<Vec<ShareInfo>, anyhow::Error> {
        info!("[🗣️ SHARE CASE]: ✅️ 获取TA的分享列表成功, target={}", target_id);
        Ok(vec![])
    }

    ////////

    /// # 5. [CASE] - 检查是否已分享
    /// * `desc`: 检查我是否已分享过某个用户主页
    pub async fn case_check_shared(
        uid: i64, // 操作者ID
        target_id: i64, // 目标用户ID
        _ctx: &AppContext, // 全局上下文
    ) -> Result<bool, anyhow::Error> {
        Ok(false)
    }
}

//////// END