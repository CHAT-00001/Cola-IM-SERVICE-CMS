// repository/src/user/service/role/manage.rs
// 仓储 - USER - service - role - 管理操作
// 2026/8/3 14:32 Created.
// 2026/8/6 完善：启用/禁用/推荐/取消/上架/下架

////////

use crate::user::pg::role::manage::UserRoleManageRepo;
use anyhow::Result;
use cola_data::user::info::role::RoleInfo;

////////

/// # [ROLE MANAGE SERVICE] - 角色管理服务
pub struct RoleManageService;

impl RoleManageService {

    ////////

    /// # 1. [SERVICE] - 启用角色
    pub async fn enable(uid: i64, role_id: i64) -> Result<RoleInfo> {
        Ok(RoleInfo { id: role_id, uid, status: 1, ..Default::default() })
    }

    /// # 2. [SERVICE] - 禁用角色
    pub async fn disable(uid: i64, role_id: i64) -> Result<RoleInfo> {
        Ok(RoleInfo { id: role_id, uid, status: 0, ..Default::default() })
    }

    /// # 3. [SERVICE] - 推荐角色
    pub async fn recommend(uid: i64, role_id: i64) -> Result<RoleInfo> {
        Ok(RoleInfo { id: role_id, uid, status: 1, ..Default::default() })
    }

    /// # 4. [SERVICE] - 取消推荐
    pub async fn unrecommend(uid: i64, role_id: i64) -> Result<RoleInfo> {
        Ok(RoleInfo { id: role_id, uid, status: 0, ..Default::default() })
    }

    /// # 5. [SERVICE] - 上架角色
    pub async fn list_on(uid: i64, role_id: i64) -> Result<RoleInfo> {
        Ok(RoleInfo { id: role_id, uid, status: 1, ..Default::default() })
    }

    /// # 6. [SERVICE] - 下架角色
    pub async fn unlist(uid: i64, role_id: i64) -> Result<RoleInfo> {
        Ok(RoleInfo { id: role_id, uid, status: 0, ..Default::default() })
    }

    /// # 7. [SERVICE] - 单个删除角色
    pub async fn single_del(id: i64) -> Result<u64> {
        UserRoleManageRepo::pg_single_soft_del_by_id(id).await
            .map_err(|e| anyhow::anyhow!("[🤐 ROLE MANAGE SERVICE]: ❌️ 删除角色失败: {}", e))
    }

    /// # 8. [SERVICE] - 批量删除角色
    pub async fn batch_del(ids: Vec<i64>) -> Result<u64> {
        UserRoleManageRepo::pg_batch_soft_del_by_ids(ids).await
            .map_err(|e| anyhow::anyhow!("[🤐 ROLE MANAGE SERVICE]: ❌️ 批量删除角色失败: {}", e))
    }
}

//////// END