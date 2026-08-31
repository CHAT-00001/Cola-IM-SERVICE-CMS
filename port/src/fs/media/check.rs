// port/src/fs/media/check.rs
// 🔌 端口 - FS - 媒体 - 检查
// 2026/8/14 13:00 Created.

////////

use anyhow::Result;

////////

/// # [CHECK PORT] - 媒体检查
/// * `desc`: `FS - 媒体检查端口`
#[async_trait::async_trait]
pub trait MediaCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 检查媒体存在
    async fn check_media_exists(&self, media_id: i64) -> Result<bool>;

    ////////

    /// # 2. [PORT] - 检查媒体可用
    async fn check_media_available(&self, media_id: i64) -> Result<bool>;
}

//////// END
