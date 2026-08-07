// repo_adapter/src/video/view/del.rs
// 🔌 插头 - 可乐视频 - 浏览 - 删除服务
// 2026/8/6 19:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::video::info::video::VideoInfo;
use cola_data::video::info::view::VideoViewInfo;
use cola_data::video::port::view::del::VideoViewDelPort;

////////

/// # [DEL SERVICE] - 删除
/// * `desc`: `视频浏览删除服务`
pub struct ViewDelService;

// 构造实现
#[async_trait]
impl VideoViewDelPort for ViewDelService {
    //

    ////////

    /// # 1. [SERVICE] - 单个
    /// * `desc`: `单个软删除`
    async fn save_view_record_update_views_count(&self, uid: i64, video_id: i64) -> Result<()> {
        todo!()
    }

    async fn view_done_update_done_count(
        &self,
        uid: i64,
        video_id: i64,
        is_done: bool,
    ) -> Result<()> {
        todo!()
    }

    async fn single_del_view_record_by_id(&self, id: i64) -> Result<(u16)> {
        todo!()
    }

    async fn batch_del_view_record_by_ids(&self, ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }

    async fn get_video_list_by_ids(&self, video_ids: Vec<i64>) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_my_viewed_list(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_here_viewed_list(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoInfo>)> {
        todo!()
    }

    async fn get_video_viewed_list(
        &self,
        video_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoViewInfo>)> {
        todo!()
    }
}

//////// END
