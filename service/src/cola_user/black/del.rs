// service/src/user/black/del.rs
// 👤 服务 - 可乐用户 - 黑名单 - 软删除服务
// 2026/8/8 03:44 Created.

////////

// /del.rs
//
// 2026/8/7 21:29 Created.

////////

use anyhow::Result;
use repository::user::pg::black::add::UserBlackAddRepo;
use repository::user::pg::black::del::UserBlackDelRepo;
use repository::user::pg::role::manage::UserRoleManageRepo;

////////

/// # [ROLE DEL SERVICE] - 软删除
/// * `desc`: `用户黑名单软删除服务`
/// * `condition`: `⚠️ 管理员身份`
pub struct BlackDelService;

// 构造实现
impl BlackDelService {
    //

    ////////

    /// # 1. [SERVICE] - 单个软删除
    /// * `desc`: `单个删除角色`
    pub async fn single_del(
        uid: i64, // 操作者 ID
        id: i64,  // 角色 ID
    ) -> Result<u64> {
        UserBlackDelRepo::single_soft_del_by_id(id)
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
        UserBlackDelRepo::batch_soft_del_by_ids(ids)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ROLE MANAGE SERVICE]: ❌️ 批量删除角色失败: {}", e))
    }
}

//////// END
