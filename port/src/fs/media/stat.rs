// port/src/fs/media/stat.rs
// 🔌 端口 - FS - 媒体 - 统计
// 2026/8/14 13:00 Created.

////////

use anyhow::Result;

////////

/// # [STAT PORT] - 媒体统计
/// * `desc`: `FS - 媒体统计端口`
#[async_trait::async_trait]
pub trait MediaStatPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 用户媒体统计
    async fn stat_user_media_count(
        &self,
        uid: i64,
    ) -> Result<u64>;

    ////////

    /// # 2. [PORT] - 应用媒体统计
    async fn stat_app_media_count(
        &self,
        app_id: String,
    ) -> Result<u64>;
}

//////// END
