// repo_adapter/src/fs/bucket/mod.rs
// 🔌 适配器 - FS - 存储桶 - music
// 2026/8/8 Created.

////////

use port::fs::bucket::FsBucketPort;
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
/// * `desc`: FS存储桶端口构造器
pub fn build_fs_bucket_port() -> FsBucketPort {
    FsBucketPort {
        add: Arc::new(add::BucketAddAdapter),
        check: Arc::new(check::BucketCheckAdapter),
        del: Arc::new(del::BucketDelAdapter),
        get: Arc::new(get::BucketGetAdapter),
        list: Arc::new(list::BucketListAdapter),
        manage: Arc::new(manage::BucketManageAdapter),
        stat: Arc::new(stat::BucketStatAdapter),
    }
}

//////// END
