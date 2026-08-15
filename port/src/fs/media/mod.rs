// port/src/fs/media/mod.rs
// ⏩️ 端口 - FS - 媒体文件 - 模块
// 2026/8/5 15:11 Created.

////////

use crate::fs::media::add::MediaAddPort;
use crate::fs::media::check::MediaCheckPort;
use crate::fs::media::del::MediaDelPort;
use crate::fs::media::get::MediaGetPort;
use crate::fs::media::list::MediaListPort;
use crate::fs::media::manage::MediaManagePort;
use crate::fs::media::stat::MediaStatPort;
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

/// # [S3 MEDIA PORT]
/// * `desc`: `S3 FS - 媒体 Ports`
#[derive(Clone)]
pub struct FsMediaPort {
    pub add: Arc<dyn MediaAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn MediaCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn MediaDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn MediaGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn MediaListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn MediaManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn MediaStatPort + Send + Sync + 'static>, // 统计
}

//////// END
