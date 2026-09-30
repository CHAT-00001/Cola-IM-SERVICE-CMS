// repo_adapter/src/fs/media/check.rs
// 🔌 适配器 - FS - 媒体文件 - 检查
// 2026/8/14 15:00 Updated.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::fs::media::check::MediaCheckPort;

////////

/// # [CHECK ADAPTER] - 检查
/// * `desc`: `FS - 媒体检查适配器`
#[derive(Debug, Default, Clone)]
pub struct MediaCheckAdapter;

#[async_trait]
impl MediaCheckPort for MediaCheckAdapter {
    ////////

    /// # 1. [ADAPTER] - 检查媒体存在
    async fn check_media_exists(&self, _media_id: i64) -> Result<bool> {
        // TODO: 实现媒体存在检查逻辑
        todo!("check_media_exists")
    }

    ////////

    /// # 2. [ADAPTER] - 检查媒体可用
    async fn check_media_available(&self, _media_id: i64) -> Result<bool> {
        // TODO: 实现媒体可用检查逻辑
        todo!("check_media_available")
    }
}

//////// END
