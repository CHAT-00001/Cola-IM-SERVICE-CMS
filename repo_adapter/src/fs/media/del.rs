// repo_adapter/src/fs/media/del.rs
// 🔌 适配器 - FS - 媒体文件 - 逻辑删除
// 2026/8/14 15:00 Updated.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::fs::media::del::MediaDelPort;

////////

/// # [DEL ADAPTER] - 删除
/// * `desc`: `FS - 媒体逻辑删除适配器`
#[derive(Debug, Default, Clone)]
pub struct MediaDelAdapter;

#[async_trait]
impl MediaDelPort for MediaDelAdapter {
    ////////

    /// # 1. [ADAPTER] - 单个删除媒体
    async fn delete_media(
        &self,
        _uid: i64,
        _media_id: i64,
    ) -> Result<u64> {
        // TODO: 实现单个媒体删除逻辑
        todo!("delete_media")
    }

    ////////

    /// # 2. [ADAPTER] - 批量删除媒体
    async fn batch_delete_medias(
        &self,
        _uid: i64,
        _media_ids: Vec<i64>,
    ) -> Result<u64> {
        // TODO: 实现批量媒体删除逻辑
        todo!("batch_delete_medias")
    }
}

//////// END