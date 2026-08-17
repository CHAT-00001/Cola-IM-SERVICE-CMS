// repo_adapter/src/fs/bucket/add.rs
// 🔌 适配器 - FS - 存储桶 - 创建
// 2026/8/14 18:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_fs::entity::bucket::BucketEntity;
use cola_data::cola_fs::command::bucket::CreateBucketCmd;
use port::fs::bucket::add::BucketAddPort;
use repository::cola_fs::pg::bucket::BucketRepo;
use repository::pg_pool;

////////

/// # [ADD ADAPTER] - 存储桶创建适配器
/// * `desc`: `FS - 存储桶创建实现`
#[derive(Debug, Default, Clone)]
pub struct BucketAddAdapter;

#[async_trait]
impl BucketAddPort for BucketAddAdapter {
    //
    
    ////////

    /// # 1. [ADAPTER] - 创建存储桶
    async fn create_bucket(&self, cmd: CreateBucketCmd) -> Result<BucketEntity> {
        let app_id = cmd.app_id.clone().unwrap_or_default();
        if !app_id.trim().is_empty()
            && BucketRepo::exists_by_app_id(&pg_pool(), &app_id, None).await?
        {
            return Err(anyhow::anyhow!("存储桶 app_id 已存在: {}", app_id));
        }

        let bucket = BucketRepo::create(&pg_pool(), cmd).await?;

        tracing::info!("[🔌 ADAPTER] - ✅️ 存储桶创建成功: app_id={}, bucket_id={}", app_id, bucket.id);

        Ok(bucket)
    }

    ////////

    /// # 2. [ADAPTER] - 生成预签名 URL
    async fn get_presigned_url(
        &self,
        uid: i64,        // 用户 ID
        bucket: &str,    // S3 bucket 名称
        object_key: &str, // 对象键
        expires_in: u64, // 有效期（秒）
    ) -> Result<String> {
        // TODO: 调用 S3 SDK 生成预签名 URL
        // 这里简化实现，返回一个示例 URL
        let presigned_url = format!(
            "https://{}.s3.amazonaws.com/{}?X-Amz-Expires={}",
            bucket, object_key, expires_in
        );

        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 生成预签名URL成功: uid={}, bucket={}, expires_in={}",
            uid, bucket, expires_in
        );

        Ok(presigned_url)
    }
}

//////// END
