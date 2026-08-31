// repo_adapter/src/cola_video/share/list.rs
// 🔌 插头 - 可乐视频 - 分享 - 列表
// 2026/8/6 18:57 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::cola_video::share::list::VideoShareListPort;

////////

/// # [ADAPTER] - share list
/// * `DESC`: `▶ 视频 - 视频分享列表适配器`
#[derive(Debug, Default, Clone)]
pub struct VideoShareListAdapter;

#[async_trait]
impl VideoShareListPort for VideoShareListAdapter {
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

//////// END
