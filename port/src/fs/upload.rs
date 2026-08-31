// port/src/fs/upload.rs
// 🔌 端口 - FS - 通用上传
// 2026/8/17 Created.

////////

use anyhow::Result;
use cola_data::cola_fs::command::upload::{CreateUploadSessionRequest, UploadPolicy};

////////

/// # [PORT] - 通用上传会话
/// * `desc`: `业务模块复用，FS 根据 app_id 映射 Bucket 并生成凭证`
#[async_trait::async_trait]
pub trait UploadSessionPort: Send + Sync {
    /// # 1. [PORT] - 创建上传会话
    /// * `desc`: `生成 session、Object Key 和短期预签名 URL`
    async fn create_session(
        &self,
        uid: i64,
        request: CreateUploadSessionRequest,
        policy: UploadPolicy,
    ) -> Result<serde_json::Value>;
}

//////// END
