// cola_infra/src/s3.rs
// 2025-12-10 15:00

////////

use anyhow::Result;
use aws_sdk_s3::Client;
use aws_sdk_s3::types::{Delete, ObjectIdentifier};
use tracing::{error, info};

////////

/// # S3 客户端
///
/// 封装了 AWS S3 的常用操作。
#[derive(Debug)]
pub struct S3Client {
    client: Client,
    bucket: String,
}

//

impl S3Client {
    /// ## 创建一个新的 S3 客户端
    ///
    /// `load_from_env()` 在当前版本中不会失败，因此移除 Result
    ///
    /// ### 参数
    /// - `bucket`: S3 存储桶的名称
    pub async fn new(bucket: String) -> Self {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        Self { client, bucket }
    }

    /// ## 批量删除 S3 对象
    ///
    /// ### 参数
    /// - `keys`: 要删除的对象的 key 列表
    ///
    /// ### 返回
    /// - `Result<()>`: 使用 `anyhow::Result` 统一处理错误
    pub async fn delete_objects(&self, keys: &[&str]) -> Result<()> {
        if keys.is_empty() {
            info!("No objects to delete.");
            return Ok(());
        }

        let objects_to_delete: Vec<ObjectIdentifier> = keys
            .iter()
            .map(|&key| ObjectIdentifier::builder().key(key).build())
            .collect::<Result<Vec<_>, _>>()?;

        let delete_request = Delete::builder()
            .set_objects(Some(objects_to_delete))
            .build()?;

        info!(
            "Deleting {} objects from bucket '{}'...",
            keys.len(),
            self.bucket
        );

        match self
            .client
            .delete_objects()
            .bucket(&self.bucket)
            .delete(delete_request)
            .send()
            .await
        {
            Ok(output) => {
                // 💡 修正：根据编译器报错，`deleted()` 和 `errors()` 返回 slice，而不是 Option。
                // 直接检查 slice 是否为空。
                let deleted = output.deleted();
                if !deleted.is_empty() {
                    info!("Successfully deleted {} objects.", deleted.len());
                }

                let errors = output.errors();
                if !errors.is_empty() {
                    for error in errors {
                        error!(
                            "Failed to delete object: key='{}', code='{}', message='{}'",
                            error.key().unwrap_or_default(),
                            error.code().unwrap_or_default(),
                            error.message().unwrap_or_default()
                        );
                    }
                }
                Ok(())
            }
            Err(e) => {
                error!("Failed to send delete_objects request: {:?}", e);
                Err(e.into())
            }
        }
    }
}