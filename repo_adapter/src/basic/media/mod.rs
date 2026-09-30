// repo_adapter/src/basic/media/mod.rs -- 适配器 - FS - 媒体 - mod
// 2026/8/8 Created.

////////

use std::sync::Arc;
use port::basic::media::MediaPort;

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

/// # [BUILD] - 构建 Meida Port
/// * `desc`: `COLA BASIC - Media Service Adapter.`
pub fn build_basic_meida_port() -> MediaPort {
    MediaPort {
        add: Arc::new(add::MediaAddAdapter),
        check: Arc::new(check::MediaCheckAdapter),
        del: Arc::new(del::MediaDelAdapter),
        get: Arc::new(get::MediaGetAdapter),
        list: Arc::new(list::MediaListAdapter),
        manage: Arc::new(manage::MediaManageAdapter),
        stat: Arc::new(stat::MediaStatAdapter),
    }
}

//////// END
