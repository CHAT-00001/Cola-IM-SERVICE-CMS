// service/src/cola_user/role/del.rs
// 👤 服务 - 可乐用户 - role - 软删除服务
// 2026/8/7 21:29 Created.

////////

use anyhow::Result;
use repository::cola_user::pg::role::manage::UserRoleManageRepo;

////////

/// # [ROLE DEL SERVICE] - 软删除
/// * `desc`: `用户角色删除服务`
/// * `condition`: `⚠️ 管理员身份`
pub struct RoleDelService;

// 构造实现
impl RoleDelService {
    //

    ////////

    /// # 1. [SERVICE] - 单个软删除
    /// * `desc`: `单个删除角色`
    pub async fn single_del(
        uid: i64, // 操作者 ID
        id: i64,  // 角色 ID
    ) -> Result<u64> {
        UserRoleManageRepo::pg_single_soft_del_by_id(id)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ROLE MANAGE SERVICE]: ❌️ 删除角色失败: {}", e))
    }

    ////////

    /// # 2. [SERVICE] - 批量软删除
    /// * `desc`: `批量删除角色`
    pub async fn batch_del(
        uid: i64,      // 操作者 ID
        ids: Vec<i64>, // 角色 IDs
    ) -> Result<u64> {
        UserRoleManageRepo::pg_batch_soft_del_by_ids(ids)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ROLE MANAGE SERVICE]: ❌️ 批量删除角色失败: {}", e))
    }
}

//////// END
