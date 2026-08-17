// port/src/fs/bucket/get.rs
// 🔌 端口 - FS - 存储桶 - 获取
// 2026/8/14 18:00 Created.

////////

use async_trait::async_trait;
use anyhow::Result;
use cola_data::cola_fs::info::bucket::BucketInfo;

////////

/// # [PORT] - 存储桶查询
/// * `desc`: `文件存储系统 - 存储桶查询端口`
#[async_trait]
pub trait BucketGetPort: Send + Sync {
    ////////

    /// # 1. [PORT] - 按 app_id 查询
    /// * `desc`: `根据应用 ID 查询存储桶配置`
    async fn get_bucket_by_app_id(
        &self,
        app_id: &str, // 应用 ID
    ) -> Result<Option<BucketInfo>>;

    ////////

    /// # 2. [PORT] - 按 ID 查询
    /// * `desc`: `根据存储桶 ID 查询`
    async fn get_bucket_by_id(
        &self,
        bucket_id: i64, // 存储桶 ID
    ) -> Result<Option<BucketInfo>>;
}
