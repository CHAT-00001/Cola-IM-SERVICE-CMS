// repo_adapter/src/fs/file/music.rs
// 2026/8/8 Created.

////////

use port::fs::file::FsFilePort;
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
/// * `desc`: FS 文件端口构造器
pub fn build_fs_file_port() -> FsFilePort {
    FsFilePort {
        add: Arc::new(add::FileAddAdapter),
        check: Arc::new(check::FileCheckAdapter),
        del: Arc::new(del::FileDelAdapter),
        get: Arc::new(get::FileGetAdapter),
        list: Arc::new(list::FileListAdapter),
        manage: Arc::new(manage::FileManageAdapter),
        stat: Arc::new(stat::FileStatAdapter),
    }
}

//////// END
