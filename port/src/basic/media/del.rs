// port/src/fs/media/del.rs
// 🔌 端口 - FS - 媒体 - 删除
// 2026/8/14 13:00 Created.

////////

use anyhow::Result;

////////

/// # [DEL PORT] - 媒体删除
/// * `desc`: `FS - 媒体逻辑删除端口`
#[async_trait::async_trait]
pub trait MediaDelPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个删除媒体
    async fn delete_media(&self, uid: i64, media_id: i64) -> Result<u64>;

    ////////

    /// # 2. [PORT] - 批量删除媒体
    async fn batch_delete_medias(&self, uid: i64, media_ids: Vec<i64>) -> Result<u64>;
}

//////// END
