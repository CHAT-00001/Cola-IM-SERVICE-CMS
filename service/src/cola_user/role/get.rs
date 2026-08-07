// service/src/cola_user/role/get.rs
// 服务 - 可乐用户 - role - 获取
// 2026/8/3 14:32 Created.

////////

use cola_data::cola_user::entity::role::UserRoleEntity;
use repository::cola_user::pg::role::get::UserRoleGetRepo;

////////

/// # [ROLE GET SERVICE] - 查询
/// * `desc`: `用户 角色 前台 服务`
pub struct UserRoleGetService;

// 构造实现
impl UserRoleGetService {
    //

    ////////

    /// # 1. [SERVICE] - 查找最新的角色列表
    pub async fn get_new_role_list(
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> Result<Vec<UserRoleEntity>, anyhow::Error> {
        // 1. 缓存

        // 2. 数据库
        UserRoleGetRepo::pg_find_new_role_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[👤 SERVICE]: ❌️ 获取最新角色列表失败: {}", e))
    }
}

//////// END
