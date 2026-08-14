// service/src/user/profile/del.rs
// 服务 - 可乐用户 - 资料 - 软删除服务
// 2026/8/7 21:36 Created.

////////

use anyhow::Result;
use repository::user::pg::role::manage::UserRoleManageRepo;

////////

/// # [PROFILE DEL SERVICE] - 软删除
/// * `desc`: `用户资料删除服务`
/// * `condition`: `⚠️ 管理员身份`
pub struct ProfileDelService;

// 构造实现
impl ProfileDelService {
    //

    ////////

    /// # 1. [SERVICE] - 单个软删除
    /// * `desc`: `单个删除资料`
    pub async fn single_del(
        uid: i64, // 操作者 ID
        id: i64,  // 资料 ID
    ) -> Result<u64> {
        UserRoleManageRepo::pg_single_soft_del_by_id(id)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ROLE MANAGE SERVICE]: ❌️ 删除资料失败: {}", e))
    }

    ////////

    /// # 2. [SERVICE] - 批量软删除
    /// * `desc`: `批量删除资料`
    pub async fn batch_del(
        uid: i64,      // 操作者 ID
        ids: Vec<i64>, // 资料 IDs
    ) -> Result<u64> {
        UserRoleManageRepo::pg_batch_soft_del_by_ids(ids)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ROLE MANAGE SERVICE]: ❌️ 批量删除资料失败: {}", e))
    }
}

//////// END
