// port/src/fs/cdn/music.rs
// ⏩️ 端口 - FS - CDN - 模块
// 2026/8/5 15:11 Created.

////////

use crate::fs::cdn::add::CdnAddPort;
use crate::fs::cdn::check::CdnCheckPort;
use crate::fs::cdn::config::CdnConfigPort;
use crate::fs::cdn::del::CdnDelPort;
use crate::fs::cdn::get::CdnGetPort;
use crate::fs::cdn::list::CdnListPort;
use crate::fs::cdn::manage::CdnManagePort;
use crate::fs::cdn::stat::CdnStatPort;
use std::sync::Arc;

////////
pub mod add; // 发布
pub mod check; // 检查
pub mod config; // 配置管理
pub mod del; // 删除
pub mod get; // 获取
pub mod list; // 列表
pub mod manage; // 管理
pub mod stat; // 统计

////////

/// # [S3 FILE PORT]
/// * `desc`: `S3 FS - 文件 Ports`
#[derive(Clone)]
pub struct FsCdnPort {
    pub add: Arc<dyn CdnAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn CdnCheckPort + Send + Sync + 'static>, // 检查
    pub config: Arc<dyn CdnConfigPort + Send + Sync + 'static>, // 配置管理
    pub del: Arc<dyn CdnDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn CdnGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn CdnListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn CdnManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn CdnStatPort + Send + Sync + 'static>, // 统计
}

//////// END
