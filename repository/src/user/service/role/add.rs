// repository/src/user/service/role/add.rs
// 仓储 - USER - service - role - 添加角色
// 2026/8/3 14:32 Created.
// 2026/8/6 实现

////////

use crate::user::pg::role::add::UserRoleAddRepo;
use anyhow::{Result, anyhow};
use cola_data::user::info::role::RoleInfo;

////////

/// # [ROLE SERVICE] - 角色服务
pub struct RoleService;

impl RoleService {

    ////////

    /// # 1. [SERVICE] - 添加角色
    /// * `desc`: 管理员添加新角色
    pub async fn add_role(
        uid: i64, // 管理员ID
        name: &str, // 角色名称
        remark: &str, // 备注
    ) -> Result<RoleInfo, anyhow::Error> {
        let _rows = UserRoleAddRepo::pg_save_new_role_record(uid, 0, remark.to_string(), 1)
            .await
            .map_err(|e| anyhow!("[🤐 ROLE SERVICE]: ❌️ 保存角色记录失败: {}", e))?;

        let info = RoleInfo {
            id: 0,
            uid,
            icon: None,
            name: Some(name.to_string()),
            name_zh: Some(name.to_string()),
            remark: Some(remark.to_string()),
            status: 1,
            add_time: 0,
            upd_time: 0,
        };

        tracing::info!("[🗣️ ROLE SERVICE]: ✅️ 角色添加成功, uid={}, name={}", uid, name);
        Ok(info)
    }

    ////////

    /// # 2. [SERVICE] - 获取角色列表
    /// * `desc`: 获取所有可用的角色列表
    pub async fn get_role_list(
        offset: i64, // 分页偏移
        limit: i64, // 每页数量
    ) -> Result<Vec<RoleInfo>, anyhow::Error> {
        let entities = crate::user::pg::role::get::UserRoleGetRepo::pg_find_new_role_list(limit, offset)
            .await
            .map_err(|e| anyhow!("[🤐 ROLE SERVICE]: ❌️ 查询角色列表失败: {}", e))?;

        let infos: Vec<RoleInfo> = entities.into_iter()
            .map(|e| RoleInfo {
                id: e.id,
                uid: e.uid,
                icon: e.icon,
                name: e.name,
                name_zh: e.name_zh,
                remark: e.remark,
                status: e.status,
                add_time: e.add_time,
                upd_time: e.upd_time,
            })
            .collect();

        tracing::info!("[🗣️ ROLE SERVICE]: ✅️ 角色列表查询成功, count={}", infos.len());
        Ok(infos)
    }
}


