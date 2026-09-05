// repo_adapter/src/fs/cdn/get.rs -- 适配器 - FS - CDN - 获取IDs
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::fs::cdn::get::CdnGetPort;

////////

/// # [GET ADAPTER] - 获取
/// * `desc`: `FS - CDN获取适配器`
#[derive(Debug, Default, Clone)]
pub struct CdnGetAdapter;

#[async_trait]
impl CdnGetPort for CdnGetAdapter {
    async fn get_comment_by_user_id(
        &self,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoCommentInfo>)> {
        todo!()
    }

    async fn get_comment_by_video(
        &self,
        video_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoCommentInfo>)> {
        todo!()
    }
}
