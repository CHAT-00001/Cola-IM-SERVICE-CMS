// repo_adapter/src/cola_video/view/get.rs
// 🔌 插头 - 可乐视频 - 浏览 - 获取服务
// 2026/8/6 19:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::video::VideoInfo;
use port::cola_video::view::get::VideoViewGetPort;

////////

/// # [GET ADAPTER] - 获取
/// * `desc`: `VIDEO - 视频浏览获取服务`
pub struct VideoViewGetAdapter;

// 构造实现
#[async_trait]
impl VideoViewGetPort for VideoViewGetAdapter {
    async fn get_video_info_by_id(&self, id: i64) -> Result<(VideoInfo)> {
        todo!()
    }

    async fn get_video_infos_by_ids(&self, ids: Vec<i64>) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }
    //

    ////////

    /// # 1. [ADAPTER] - 获取用户浏览过的视频IDs
    async fn get_video_ids_by_user_id(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
