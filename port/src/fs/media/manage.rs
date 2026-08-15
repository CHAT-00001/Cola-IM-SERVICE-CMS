// port/src/fs/media/manage.rs
// 🔌 端口 - FS - 媒体 - 管理
// 2026/8/14 13:00 Created.

////////

use anyhow::Result;
use cola_data::cola_fs::entity::media::MediaEntity;

////////

/// # [MANAGE PORT] - 媒体管理
/// * `desc`: `FS - 媒体管理端口`
#[async_trait::async_trait]
pub trait MediaManagePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 更新媒体状态
    async fn update_media_status(
        &self,
        media_id: i64,
        status: i16,
    ) -> Result<()>;

    ////////

    /// # 2. [PORT] - 更新转码成功信息
    async fn update_transcode_success(
        &self,
        media_id: i64,
        hls_playlist_url: Option<String>,
        variants_meta: Option<String>,
    ) -> Result<()>;
}

//////// END