// port/src/fs/bucket/add.rs
// 🔌 端口 - FS - 存储桶 - 创建
// 2026/8/14 18:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_fs::command::bucket::CreateBucketCmd;
use cola_data::cola_fs::entity::bucket::BucketEntity;

////////

/// # [PORT] - 存储桶创建
/// * `desc`: `文件存储系统 - 存储桶创建端口`
#[async_trait]
pub trait BucketAddPort: Send + Sync {
    ////////

    /// # 1. [PORT] - 创建存储桶
    /// * `desc`: `创建一个新的存储桶配置`
    async fn create_bucket(&self, cmd: CreateBucketCmd) -> Result<BucketEntity>;

    ////////

    /// # 2. [PORT] - 生成预签名 URL
    /// * `desc`: `为上传生成预签名 URL`
    async fn get_presigned_url(
        &self,
        uid: i64,         // 用户 ID
        bucket_id: i64,   // 存储桶 ID，适配器内部读取敏感配置
        object_key: &str, // 对象键
        expires_in: u64,  // 有效期（秒）
    ) -> Result<String>;
}

//////// END
