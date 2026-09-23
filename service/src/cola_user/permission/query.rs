// service/src/cola_user/permission/query.rs -- 服务 - USER - 权限 - 查询
// 2026/9/19 00:59 Created.

////////

use anyhow::Result;
use cola_data::cola_user::info::permission::UserPermissionContext;
use tracing::warn;

////////

/// # [PERMISSION QUERY SERVICE] - 权限查询服务
/// * `desc`: `查询用户权限上下文（从cola_user权限中心）`
pub struct UserPermissionQueryService;

impl UserPermissionQueryService {
    ////////

    /// # 1. [SERVICE] - 获取用户权限上下文（已登录用户）
    /// * `desc`: `网关层调用，按 uid 聚合用户权限信息`
    pub async fn get_user_permission_context(
        uid: i64, // 用户ID
    ) -> Result<UserPermissionContext> {
        // 🚧 TODO: 实现权限查询逻辑
        // 1. 查询用户权限等级
        // 2. 查询用户角色
        // 3. 根据等级和角色聚合权限
        // 4. 缓存到 Redis

        // ⚠️ 临时实现：为登录用户返回权限等级 2（正常用户）
        // TODO: 实际应该查询 cola_user 的权限配置表
        warn!(
            "[🔌 ADAPTER]: ⚠️ 权限查询未完全实现, 返回临时权限(等级=2) - uid={}",
            uid
        );

        // 为了支持发弹幕等操作，登录用户默认返回权限等级 2
        Ok(UserPermissionContext {
            uid,
            base_level: 2, // ✨ 登录用户默认权限 = 2
            level_name: "USER".to_string(),
            roles: vec![],
            sys_permissions: vec![],
            is_vip: false,
            is_creator: false,
            is_operator: false,
            is_admin: false,
        })
    }

    ////////

    /// # 2. [SERVICE] - 游客权限兜底（权限=1）
    /// * `desc`: `auth 缺失 / 为空 / token 无效时，从权限中心返回游客兜底权限（等级=1）`
    pub fn guest_permission_context() -> UserPermissionContext {
        warn!("[🔌 ADAPTER]: 👤 auth 缺失或无效，返回权限中心游客兜底 - 权限等级=1");
        UserPermissionContext::guest()
    }

    ////////

    /// # 3. [SERVICE] - 统一解析权限上下文（含游客兜底）
    /// * `desc`: `uid<=0 直接游客兜底(1)；uid>0 查询，查询失败同样兜底(1)`
    /// * `condition`: `网关层唯一入口，auth 空/错误 与 正常登录 共用同一解析路径`
    pub async fn resolve_permission_context(
        uid: i64, // 用户ID: <=0 视为游客
    ) -> UserPermissionContext {
        // 1. 游客（auth 空 / token 无效）：直接拿权限中心游客兜底
        if uid <= 0 {
            return Self::guest_permission_context();
        }

        // 2. 已登录：查询权限中心，失败则降级为游客兜底（权限=1）
        match Self::get_user_permission_context(uid).await {
            Ok(ctx) => ctx,
            Err(e) => {
                warn!(
                    "[🔌 ADAPTER]: ❌️ 权限查询失败，降级为游客兜底 - 权限等级=1: {}",
                    e
                );
                UserPermissionContext::guest()
            }
        }
    }

    ////////

    /// # 4. [SERVICE] - 检查权限等级
    pub async fn check_level(
        uid: i64,              // 用户ID
        required_level: i16,   // 需要的权限等级
    ) -> Result<bool> {
        let ctx = Self::get_user_permission_context(uid).await?;
        Ok(ctx.has_level(required_level))
    }

    ////////
}

//////// END
