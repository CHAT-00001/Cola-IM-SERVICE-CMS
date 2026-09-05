// repo_adapter/src/user/black/mod.rs -- 适配器 - USER - 黑名单 - mod
// 2026-08-02 Created.

////////

use port::cola_user::black::UserBlackPort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod check; // 检查
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILDER] - 构造用户黑名单端口
/// * `DESC`: `COLA USER - Black Ports.`
pub fn build_user_black_port() -> UserBlackPort {
    UserBlackPort {
        add: Arc::new(add::BlackAddAdapter),
        check: Arc::new(check::BlackCheckAdapter),
        del: Arc::new(del::BlackDelAdapter),
        get: Arc::new(get::BlackGetAdapter),
        list: Arc::new(list::BlackListAdapter),
        manage: Arc::new(manage::BlackManageAdapter),
        stat: Arc::new(stat::UserBlackStatAdapter),
    }
}

//////// END
