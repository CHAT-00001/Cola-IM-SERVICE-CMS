// repo_adapter/src/cola_coc/video/get.rs -- 插头 - VIDEO - 视频 - 获取IDs
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::video::VideoInfo;
use port::cola_video::video::get::VideoGetPort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `🔌 视频发布插头`
pub struct VideoGetAdapter;

// 构造实现
#[async_trait]
impl VideoGetPort for VideoGetAdapter {
    async fn get_my_list(
        &self,
        uid: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
        is_liked: bool,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_he_list(
        &self,
        uid: i64,
        user_id: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_video_info_by_id(&self, uid: i64, video_id: i64) -> Result<(VideoInfo)> {
        todo!()
    }

    async fn get_video_infos_by_ids(
        &self,
        uid: i64,
        video_ids: Vec<i64>,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }
}

//////// END
