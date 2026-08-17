// repo_adapter/src/fs/cdn/mod.rs
// 🔌 适配器 - FS - CDN - mod
// 2026/8/8 Created.

////////

use port::fs::cdn::FsCdnPort;
use std::sync::Arc;

////////

pub mod add; // 发布
pub mod alive; // 存活
pub mod check; // 检查
pub mod config; // 配置管理
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [BUILD] - 构建 CDN Port
/// * `desc`: FS CDN端口构造器
pub fn build_fs_cdn_port() -> FsCdnPort {
    FsCdnPort {
        add: Arc::new(add::CdnAddAdapter),
        check: Arc::new(check::CdnCheckAdapter),
        config: Arc::new(config::CdnConfigAdapter),
        del: Arc::new(del::CdnDelAdapter),
        get: Arc::new(get::CdnGetAdapter),
        list: Arc::new(list::CdnListAdapter),
        manage: Arc::new(manage::CdnManageAdapter),
        stat: Arc::new(stat::CdnStatAdapter),
    }
}

//////// END
