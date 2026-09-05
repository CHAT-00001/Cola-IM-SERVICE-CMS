// repo_adapter/src/user/share/mod.rs  -- 适配器 - USER - 分享 - mod
// 2026/8/8 12:45 Created.

////////

use port::cola_user::share::UserSharePort;
use std::sync::Arc;

pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILDER] - 构造用户分享端口
/// * `DESC`: `COLA USER - Share Ports.`
pub fn build_user_share_port() -> UserSharePort {
    UserSharePort {
        add: Arc::new(add::UserShareAddAdapter),
        check: Arc::new(check::ShareCheckAdapter),
        del: Arc::new(del::ShareDelAdapter),
        get: Arc::new(get::ShareGetAdapter),
        list: Arc::new(list::ShareListAdapter),
        manage: Arc::new(manage::ShareManageAdapter),
        stat: Arc::new(stat::UserShareStatAdapter),
    }
}

//////// END
