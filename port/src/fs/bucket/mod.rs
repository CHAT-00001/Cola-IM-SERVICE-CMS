// port/src/fs/bucket/music.rs
// ⏩️ 端口 - FS - 存储桶 - 模块
// 2026/8/5 15:11 Created.

////////

use crate::fs::bucket::add::BucketAddPort;
use crate::fs::bucket::check::BucketCheckPort;
use crate::fs::bucket::del::BucketDelPort;
use crate::fs::bucket::get::BucketGetPort;
use crate::fs::bucket::list::BucketListPort;
use crate::fs::bucket::manage::BucketManagePort;
use crate::fs::bucket::stat::BucketStatPort;
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

/// # [S3 BUCKET PORT]
/// * `desc`: `S3 FS - 存储桶 Ports`
#[derive(Clone)]
pub struct FsBucketPort {
    pub add: Arc<dyn BucketAddPort + Send + Sync + 'static>, // 发布
    pub check: Arc<dyn BucketCheckPort + Send + Sync + 'static>, // 检查
    pub del: Arc<dyn BucketDelPort + Send + Sync + 'static>, // 删除
    pub get: Arc<dyn BucketGetPort + Send + Sync + 'static>, // 获取
    pub list: Arc<dyn BucketListPort + Send + Sync + 'static>, // 列表
    pub manage: Arc<dyn BucketManagePort + Send + Sync + 'static>, // 管理
    pub stat: Arc<dyn BucketStatPort + Send + Sync + 'static>, // 统计
}

//////// END
