// port/src/fs/media/list.rs
// 🔌 端口 - FS - 媒体 - 列表
// 2026/8/14 13:00 Created.

////////

use anyhow::Result;
use cola_data::cola_fs::entity::media::MediaEntity;

////////

/// # [LIST PORT] - 媒体列表
/// * `desc`: `FS - 媒体列表端口`
#[async_trait::async_trait]
pub trait MediaListPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 用户媒体列表
    async fn list_user_medias(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MediaEntity>>;

    ////////

    /// # 2. [PORT] - 应用媒体列表
    async fn list_app_medias(
        &self,
        app_id: String,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MediaEntity>>;
}

//////// END