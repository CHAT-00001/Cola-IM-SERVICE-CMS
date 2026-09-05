// repo_adapter/src/user/ban/mod.rs -- 适配器 - USER - 封禁 - mod
// 2026/8/7 03:32 Created.

////////

use port::cola_user::ban::UserBanPort;
use std::sync::Arc;

pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod get; //获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILDER] - 构造用户封禁端口
/// * `DESC`: `COLA USER - Ban Ports.`
pub fn build_user_ban_port() -> UserBanPort {
    UserBanPort {
        add: Arc::new(add::BanAddAdapter),
        check: Arc::new(check::BanCheckAdapter),
        del: Arc::new(del::BanDelAdapter),
        get: Arc::new(get::BanGetAdapter),
        list: Arc::new(list::BanListAdapter),
        manage: Arc::new(manage::BanManageAdapter),
        stat: Arc::new(stat::UserBankStatAdapter),
    }
}

//////// END
