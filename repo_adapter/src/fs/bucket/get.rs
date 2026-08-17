// repo_adapter/src/fs/bucket/get.rs
// 🔌 适配器 - FS - 存储桶 - 获取
// 2026/8/14 15:00 Updated.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_fs::entity::bucket::BucketEntity;
use port::fs::bucket::get::BucketGetPort;
use repository::cola_fs::pg::bucket::BucketRepo;
use repository::pg_pool;

////////

/// # [GET ADAPTER] - 获取
/// * `desc`: `FS - 存储桶获取适配器`
#[derive(Debug, Default, Clone)]
pub struct BucketGetAdapter;

#[async_trait]
impl BucketGetPort for BucketGetAdapter {
    ////////

    /// # 1. [ADAPTER] - 按 app_id 查询
    /// * `desc`: `根据应用 ID 查询存储桶配置`
    async fn get_bucket_by_app_id(
        &self,
        app_id: &str, // 应用 ID
    ) -> Result<Option<BucketEntity>> {
        let result = BucketRepo::find_by_app_id(&pg_pool(), app_id).await?;
        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 按 app_id 查询存储桶完成: app_id={}, found={}",
            app_id,
            result.is_some()
        );
        Ok(result)
    }

    ////////

    /// # 2. [ADAPTER] - 按 ID 查询
    /// * `desc`: `根据存储桶 ID 查询`
    async fn get_bucket_by_id(
        &self,
        bucket_id: i64, // 存储桶 ID
    ) -> Result<Option<BucketEntity>> {
        let result = BucketRepo::find_by_id(&pg_pool(), bucket_id).await?;
        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 按 ID 查询存储桶完成: bucket_id={}, found={}",
            bucket_id,
            result.is_some()
        );
        Ok(result)
    }
}
