// repo_adapter/src/aaaa/app/mod.rs -- ADAPTER - BASIC - APP - mod
// 2026/4/8 10:20 Created.

////////

use port::basic::app::AppPort;
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
/// * `desc`: 基础应用 服务端口
pub fn build_basic_app_port() -> AppPort {
    AppPort {
        add: Arc::new(add::AppAddAdapter),
        check: Arc::new(check::AppCheckAdapter),
        del: Arc::new(del::AppDelAdapter),
        get: Arc::new(get::AppGetAdapter),
        list: Arc::new(list::AppListAdapter),
        manage: Arc::new(manage::AppManageAdapter),
        stat: Arc::new(stat::AppStatAdapter),
    }
}

//////// END
