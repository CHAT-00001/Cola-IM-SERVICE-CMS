// repo_adapter/src/fs/media/manage.rs
// 🔌 适配器 - FS - 媒体文件 - 管理
// 2026/8/14 15:00 Updated.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::fs::media::manage::MediaManagePort;

////////

/// # [MANAGE ADAPTER] - 管理
/// * `desc`: `FS - 媒体管理适配器`
#[derive(Debug, Default, Clone)]
pub struct MediaManageAdapter;

#[async_trait]
impl MediaManagePort for MediaManageAdapter {
    ////////

    /// # 1. [ADAPTER] - 更新媒体状态
    async fn update_media_status(
        &self,
        _media_id: i64,
        _status: i16,
    ) -> Result<()> {
        // TODO: 实现媒体状态更新逻辑
        todo!("update_media_status")
    }

    ////////

    /// # 2. [ADAPTER] - 更新转码成功信息
    async fn update_transcode_success(
        &self,
        _media_id: i64,
        _hls_playlist_url: Option<String>,
        _variants_meta: Option<String>,
    ) -> Result<()> {
        // TODO: 实现转码成功更新逻辑
        todo!("update_transcode_success")
    }
}
