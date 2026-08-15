// port/src/fs/media/get.rs
// 🔌 端口 - FS - 媒体 - 获取
// 2026/8/14 13:00 Created.

////////

use anyhow::Result;
use cola_data::cola_fs::entity::media::MediaEntity;

////////

/// # [GET PORT] - 媒体获取
/// * `desc`: `FS - 媒体获取端口`
#[async_trait::async_trait]
pub trait MediaGetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 按 ID 获取媒体
    async fn get_media_by_id(
        &self,
        media_id: i64,
    ) -> Result<Option<MediaEntity>>;

    ////////

    /// # 2. [PORT] - 批量按 ID 获取媒体
    async fn batch_get_medias(
        &self,
        media_ids: Vec<i64>,
    ) -> Result<Vec<MediaEntity>>;
}

//////// END