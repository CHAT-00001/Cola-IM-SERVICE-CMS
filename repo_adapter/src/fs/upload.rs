// repo_adapter/src/fs/upload.rs
// 🔌 适配器 - FS - 通用上传
// 2026/8/17 Created.

////////

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::Utc;
use cola_data::cola_fs::command::upload::{CreateUploadSessionRequest, UploadPolicy};
use port::fs::upload::UploadSessionPort;
use repository::cola_fs::pg::bucket::BucketRepo;
use repository::pg_pool;
use uuid::{Uuid, uuid};

////////

const UPLOAD_EXPIRES_IN: u64 = 600;

////////

/// # [ADAPTER] - 通用上传适配器
/// * `desc`: `根据 app_id 查询 Bucket，统一生成 Object Key 和 S3 V4 上传 URL`
#[derive(Debug, Default, Clone)]
pub struct UploadSessionAdapter;

#[async_trait]
impl UploadSessionPort for UploadSessionAdapter {
    ////////

    /// # 1. [ADAPTER] - 创建上传会话
    /// * `desc`: `业务策略校验后生成通用上传凭证`
    async fn create_session(
        &self,
        uid: i64,
        request: CreateUploadSessionRequest,
        policy: UploadPolicy,
    ) -> Result<serde_json::Value> {
        if request.app_id != policy.app_id || request.ugc_type != policy.ugc_type {
            return Err(anyhow!("上传业务策略不匹配: app_id={}", request.app_id));
        }
        if request.idempotency_key.trim().is_empty() {
            return Err(anyhow!("缺少上传会话幂等键"));
        }
        policy.validate(&request.files)?;

        let bucket = BucketRepo::find_by_app_id(&pg_pool(), &request.app_id)
            .await?
            .ok_or_else(|| anyhow!("存储桶不存在: {}", request.app_id))?;
        if bucket.status != 1 || bucket.is_banned || bucket.is_deleted {
            return Err(anyhow!("存储桶不可用: {}", request.app_id));
        }

        let session_id = format!("us_{}_{}", request.ugc_type, Uuid::new_v4().simple());
        let mut files = Vec::with_capacity(request.files.len());
        for file in request.files {
            let extension = file.file_name.rsplit('.').next().unwrap_or("bin");
            let object_key = format!(
                "{}/{}/uid_{}/{}/{}/{}.{}",
                request.app_id,
                Utc::now().format("%Y/%m/%d"),
                uid,
                session_id,
                file.role,
                Uuid::new_v4().simple(),
                extension
            );
            let credentials = aws_sdk_s3::config::Credentials::new(
                bucket.access_key.clone(),
                bucket.secret_key.clone(),
                None,
                None,
                "cola-fs",
            );
            let config = aws_sdk_s3::config::Builder::new()
                .region(aws_sdk_s3::config::Region::new(bucket.region.clone()))
                .endpoint_url(bucket.endpoint.clone())
                .credentials_provider(credentials)
                .force_path_style(true)
                .build();
            let client = aws_sdk_s3::Client::from_conf(config);
            let presigned = client
                .put_object()
                .bucket(&bucket.bucket)
                .key(&object_key)
                .presigned(aws_sdk_s3::presigning::PresigningConfig::expires_in(
                    std::time::Duration::from_secs(UPLOAD_EXPIRES_IN),
                )?)
                .await?;
            files.push(serde_json::json!({
                "role": file.role,
                "file_name": file.file_name,
                "mime_type": file.mime_type,
                "file_size": file.file_size,
                "file_hash": file.file_hash,
                "object_key": object_key,
                "upload_url": presigned.uri().to_string(),
                "expires_in": UPLOAD_EXPIRES_IN
            }));
        }

        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 通用上传凭证生成成功: uid={}, app_id={}",
            uid,
            request.app_id
        );
        Ok(serde_json::json!({
            "session_id": session_id,
            "app_id": policy.app_id,
            "ugc_type": policy.ugc_type,
            "bucket": bucket.bucket,
            "expires_in": UPLOAD_EXPIRES_IN,
            "expired_at": Utc::now() + chrono::Duration::days(7),
            "files": files
        }))
    }
}

//////// END
