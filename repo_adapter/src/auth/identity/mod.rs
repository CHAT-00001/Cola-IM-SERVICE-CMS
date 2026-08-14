// repo_adapter/src/auth/identity/mod.rs
// 🔌 适配器 - AUTH - 身份识别 - mod
// 2026/8/8 Created.

////////

use port::auth::identity::AuthIdentityPort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod alive; // 存活
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILD] - 构建 IDENTITY Port
/// * `desc`: 验证身份端口构造器
pub fn build_auth_identity_port() -> AuthIdentityPort {
    AuthIdentityPort {
        add: Arc::new(add::IdentityAddAdapter),
        check: Arc::new(check::IdentityCheckAdapter),
        del: Arc::new(del::IdentityDelAdapter),
        get: Arc::new(get::IdentityGetAdapter),
        list: Arc::new(list::IdentityListAdapter),
        manage: Arc::new(manage::IdentityManageAdapter),
        stat: Arc::new(stat::IdentityStatAdapter),
    }
}

//////// END
