// port/src/fs/file/mod.rs
// ⏩️ 端口 - FS - 文件 - 模块
// 2026/8/5 15:11 Created.

////////

use crate::fs::file::add::FileAddPort;
use crate::fs::file::check::FileCheckPort;
use crate::fs::file::del::FileDelPort;
use crate::fs::file::get::FileGetPort;
use crate::fs::file::list::FileListPort;
use crate::fs::file::manage::FileManagePort;
use crate::fs::file::stat::FileStatPort;
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

/// # [S3 FILE PORT]
/// * `desc`: `S3 FS - 文件 Ports`
#[derive(Clone)]
pub struct FsFilePort {
    pub add: Arc<dyn FileAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn FileCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn FileDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn FileGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn FileListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn FileManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn FileStatPort + Send + Sync + 'static>, // 统计
}

//////// END
