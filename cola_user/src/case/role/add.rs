// cola_user/src/case/role/add.rs
// core - USER - case - role - 角色添加/删除 用例
// 2026/8/2 22:49 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::app::ctx::AppContext;
use cola_data::user::info::role::RoleInfo;
use cola_data::user::vo::role::RoleVo;
use repository::user::service::role::add::RoleService;
use tracing::info;

////////

/// # [ADD CASE] - 发布
/// * `desc`: `管理员发布新的角色`
/// * `condition`: `仅限运营管理员` + `系统管理员`
pub struct UserRoleAddCase;

impl UserRoleAddCase {
    //

    ////////

    /// # 1. [CASE] - 添加角色
    /// * `desc`: `管理员添加新角色`
    pub async fn case_add_new(
        uid: i64,          // 管理员ID
        name: &str,        // 角色名称
        remark: &str,      // 备注
        _ctx: &AppContext, // 全局上下文
    ) -> Result<RoleVo, anyhow::Error> {
        let info = RoleService::add_role(uid, name, remark)
            .await
            .map_err(|e| anyhow!("[🤐 ROLE CASE]: ❌️ 添加角色失败: {}", e))?;

        info!(
            "[🗣️ ROLE CASE] - ✅️ 添加角色成功: uid={}, name={}",
            uid, name
        );

        let vo = RoleVo::new(info);
        Ok(vo)
    }

    ////////

    /// # 2. [CASE] - 删除角色
    /// * `desc`: `管理员删除角色`
    pub async fn case_delete_role(
        uid: i64,          // 管理员ID
        role_id: i64,      // 角色ID
        _ctx: &AppContext, // 全局上下文
    ) -> Result<(), anyhow::Error> {
        repository::user::pg::role::add::UserRoleAddRepo::soft_delete_follows_by_uid(uid)
            .await
            .map_err(|e| anyhow!("[🤐 ROLE CASE]: ❌️ 删除角色失败: {}", e))?;

        info!(
            "[🗣️ ROLE CASE] - ✅️ 删除角色成功: uid={}, role_id={}",
            uid, role_id
        );
        Ok(())
    }
}

//////// END
