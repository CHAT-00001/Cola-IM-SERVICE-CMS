// repo_adapter/src/fs/bucket/add.rs
// 🔌 适配器 - FS - 存储桶 - 创建
// 2026/8/14 18:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_fs::command::bucket::CreateBucketCmd;
use cola_data::cola_fs::entity::bucket::BucketEntity;
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

        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 存储桶创建成功: app_id={}, bucket_id={}",
            app_id,
            bucket.id
        );

        Ok(bucket)
    }

    ////////

    /// # 2. [ADAPTER] - 生成预签名 URL
    async fn get_presigned_url(
        &self,
        uid: i64,         // 用户 ID
        bucket_id: i64,   // 存储桶 ID
        object_key: &str, // 对象键
        expires_in: u64,  // 有效期（秒）
    ) -> Result<String> {
        let bucket_config = BucketRepo::find_by_id(&pg_pool(), bucket_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("存储桶不存在: {}", bucket_id))?;
        let credentials = aws_sdk_s3::config::Credentials::new(
            bucket_config.access_key,
            bucket_config.secret_key,
            None,
            None,
            "cola-fs",
        );
        let config = aws_sdk_s3::config::Builder::new()
            .region(aws_sdk_s3::config::Region::new(
                bucket_config.region.clone(),
            ))
            .endpoint_url(bucket_config.endpoint.clone())
            .credentials_provider(credentials)
            .force_path_style(true)
            .build();
        let client = aws_sdk_s3::Client::from_conf(config);
        let presigned = client
            .put_object()
            .bucket(&bucket_config.bucket)
            .key(object_key)
            .presigned(aws_sdk_s3::presigning::PresigningConfig::expires_in(
                std::time::Duration::from_secs(expires_in),
            )?)
            .await?;
        let presigned_url = presigned.uri().to_string();

        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 生成预签名URL成功: uid={}, bucket={}, expires_in={}",
            uid,
            bucket_config.bucket,
            expires_in
        );

        Ok(presigned_url)
    }
}

//////// END
