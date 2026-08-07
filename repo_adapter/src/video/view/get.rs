// repo_adapter/src/video/view/get.rs
// 🔌 插头 - 可乐视频 - 浏览 - 获取服务
// 2026/8/6 19:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::video::info::video::VideoInfo;
use cola_data::video::info::view::VideoViewInfo;
use cola_data::video::port::view::get::VideoViewGetPort;

////////

/// # [GET SERVICE] - 获取
/// * `desc`: `视频浏览获取服务`
pub struct ViewGetService;

// 构造实现
#[async_trait]
impl VideoViewGetPort for ViewGetService {
    //

    ////////

    /// # 1. [SERVICE] - 单个
    /// * `desc`: `单个软删除`
    async fn create_new(&self, uid: i64, video_id: i64) -> Result<()> {
        todo!()
    }

    async fn update_one(&self, uid: i64, video_id: i64, is_done: bool) -> Result<()> {
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
