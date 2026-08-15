// repo_adapter/src/fs/media/mod.rs
// 2026/8/8 Created.

////////

use port::fs::media::FsMediaPort;
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
/// * `desc`: FS媒体端口构造器
pub fn build_auth_meida_port() -> FsMediaPort {
    FsMediaPort {
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
