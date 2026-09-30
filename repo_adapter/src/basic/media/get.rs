// repo_adapter/src/fs/media/get.rs
// 🔌 适配器 - FS - 媒体文件 - 获取
// 2026/8/14 15:00 Updated.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_fs::entity::media::MediaEntity;
use port::fs::media::get::MediaGetPort;
use repository::cola_fs::pg::media::MediaRepo;
use repository::pg_pool;

////////

/// # [GET ADAPTER] - 获取
/// * `desc`: `FS - 媒体文件获取适配器`
#[derive(Debug, Default, Clone)]
pub struct MediaGetAdapter;

#[async_trait]
impl MediaGetPort for MediaGetAdapter {
    ////////

    /// # 1. [ADAPTER] - 按 ID 获取媒体
    async fn get_media_by_id(&self, media_id: i64) -> Result<Option<MediaEntity>> {
        // TODO: 实现媒体获取逻辑
        Ok(MediaRepo::find_by_id(&pg_pool(), media_id).await?)
    }

    ////////

    /// # 2. [ADAPTER] - 批量按 ID 获取媒体
    async fn batch_get_medias(&self, media_ids: Vec<i64>) -> Result<Vec<MediaEntity>> {
        // TODO: 实现批量媒体获取逻辑
        let mut result = Vec::new();
        for media_id in media_ids {
            if let Some(media) = MediaRepo::find_by_id(&pg_pool(), media_id).await? {
                result.push(media);
            }
        }
        Ok(result)
    }
}
