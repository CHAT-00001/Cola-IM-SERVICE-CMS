// repo_adapter/src/fs/media/list.rs
// 🔌 适配器 - FS - 媒体文件 - 列表
// 2026/8/14 15:00 Updated.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_fs::entity::media::MediaEntity;
use port::fs::media::list::MediaListPort;

////////

/// # [LIST ADAPTER] - 列表
/// * `desc`: `FS - 媒体文件列表适配器`
#[derive(Debug, Default, Clone)]
pub struct MediaListAdapter;

#[async_trait]
impl MediaListPort for MediaListAdapter {
    ////////

    /// # 1. [ADAPTER] - 用户媒体列表
    async fn list_user_medias(
        &self,
        _uid: i64,
        _limit: i64,
        _offset: i64,
    ) -> Result<Vec<MediaEntity>> {
        // TODO: 实现用户媒体列表查询逻辑
        todo!("list_user_medias")
    }

    ////////

    /// # 2. [ADAPTER] - 应用媒体列表
    async fn list_app_medias(
        &self,
        _app_id: String,
        _limit: i64,
        _offset: i64,
    ) -> Result<Vec<MediaEntity>> {
        // TODO: 实现应用媒体列表查询逻辑
        todo!("list_app_medias")
    }
}
