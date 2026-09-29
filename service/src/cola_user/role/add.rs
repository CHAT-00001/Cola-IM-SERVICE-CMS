// service/src/user/role/add.rs -- SERVICE - USER - 角色 - 创建服务
// 2026/8/3 14:32 Created.

////////

use anyhow::{Result, anyhow};
use cola_data::cola_user::info::role::RoleInfo;
use repository::user::pg::role::add::UserRoleAddRepo;
use repository::user::pg::role::get::UserRoleGetRepo;

////////

/// # [ROLE ADD SERVICE] - 角色发布
/// * `desc`: `管理员发布角色服务`
/// * `condition`: `⚠️ 管理员身份`
pub struct RoleAddService;

// 构造实现
impl RoleAddService {
    ////////

    /// # 1. [SERVICE] - 添加角色
    /// * `desc`: 管理员添加新角色
    pub async fn add_role(
        uid: i64,     // 管理员ID
        name: &str,   // 角色名称
        remark: &str, // 备注
    ) -> Result<RoleInfo, anyhow::Error> {

        ////////

        // 💡 - PG 仓储
        let _rows = UserRoleAddRepo::pg_save_new_role_record(uid, 0, remark.to_string(), 1)
            .await
            .map_err(|e| anyhow!("[🤐 ROLE SERVICE]: ❌️ 保存角色记录失败: {}", e))?;

        ////////

        // 💡 - INFO 构造 (使用 RoleInfo 的 new 构造方法)
        let info = RoleInfo::new(uid, name.to_string(), remark.to_string());

        tracing::info!(
            "[🗣️ ROLE SERVICE]: ✅️ 角色添加成功, uid={}, name={}",
            uid,
            name
        );
        Ok(info)
    }

    ////////

    /// # 2. [SERVICE] - 获取角色列表
    /// * `desc`: 获取所有可用的角色列表
    pub async fn get_role_list(
        offset: i64, // 分页偏移
        limit: i64,  // 每页数量
    ) -> Result<Vec<RoleInfo>, anyhow::Error> {

        ////////

        // 💡 - ENTITIES - 从PG获取列表
        let entities = UserRoleGetRepo::pg_find_new_role_list(limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 ROLE SERVICE]: ❌️️ 查询角色列表失败: {}", e))?;

        ////////

        // 💡 - INFOS - 使用 RoleInfo 的 from_entity 转换方法（如果采用具体 entity 版本的实现）
        let infos: Vec<RoleInfo> = entities
            .into_iter()
            .map(|e| RoleInfo::from_entity(&e))
            .collect();

        ////////

        // 💡 - LOGGER
        tracing::info!(
            "[🗣️ ROLE SERVICE]: ✅️ 角色列表查询成功, count={}",
            infos.len()
        );
        Ok(infos)
    }
}

//////// END