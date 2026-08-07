// repository/src/user/service/role/get.rs
// 仓储 - USER - service - role - 添加
// 2026/8/3 14:32 Created.

////////

use crate::user::pg::role::get::UserRoleGetRepo;
use cola_data::user::entity::role::UserRoleEntity;

////////

/// # [GET SERVICE] - 用户 角色 前台 服务
pub struct UserRoleGetService;

// 构造函数
impl UserRoleGetService {
    //

    ////////

    /// # 1. [SERVICE] - 查找最新的角色列表
    pub async fn get_new_role_list(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserRoleEntity>, anyhow::Error> {
        // 1. 缓存

        // 2. 数据库
        UserRoleGetRepo::pg_find_new_role_list(limit, offset)
            .await
            .map_err(|e| anyhow::anyhow!("[👤 SERVICE]: ❌️ 获取最新角色列表失败: {}", e))
    }
}

//////// END
