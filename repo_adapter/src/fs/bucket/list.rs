// repo_adapter/src/fs/bucket/list.rs
// 🔌 适配器 - FS - 存储桶 -评论列表
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::fs::bucket::list::BucketListPort;

////////

/// # [LIAT ADAPTER] - 列表
/// * `desc`: `FS - 存储桶列表适配器`
#[derive(Debug, Default, Clone)]
pub struct BucketListAdapter;

#[async_trait]
impl BucketListPort for BucketListAdapter {
    async fn get_my_like_record(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }

    async fn get_he_like_record(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(VideoCommentInfo)> {
        todo!()
    }
}
