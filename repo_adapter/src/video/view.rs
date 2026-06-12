// repo_adapter/src/video/view.rs
// 2026-06-12

////////

use async_trait::async_trait;
use cola_data::video::info::video::VideoInfo;
use cola_data::video::port::view::ViewPort;
use repo::video::service::view::ViewService;

////////

/// # [VIEW PORT] - 浏览 端口 插头
pub struct ViewPortAdapter;

////////

#[async_trait]
impl ViewPort for ViewPortAdapter {
    ////////

    /// # 1. 保存浏览记录 + 更新浏览数量
    /// * `DESC`: 浏览数量递增不可逆,不能-1
    async fn save_view_record_update_views_count(
        &self,
        _uid: i64,
        _video_id: i64,
    ) -> anyhow::Result<()> {
        // TODO: call repo::video::pg::view::VideoViewRepo if needed
        Ok(())
    }

    ////////

    /// # 2. 浏览完成报告 + 更新完播数量
    async fn view_done_update_done_count(
        &self,
        _uid: i64,
        _video_id: i64,
        _is_done: bool,
    ) -> anyhow::Result<()> {
        // TODO: call repo::video::pg::view::VideoViewRepo if needed
        Ok(())
    }

    ////////

    /// # 3. [PORT] - 获取一个视频信息
    async fn get_video_list_by_id(&self, video_id: i64) -> anyhow::Result<VideoInfo> {
        ViewService::get_one_video_info(video_id).await
    }

    ////////

    /// # 4. [PORT] - 遍历视频ids获取视频信息
    async fn get_video_list_by_ids(&self, video_ids: Vec<i64>) -> anyhow::Result<Vec<VideoInfo>> {
        ViewService::batch_get_videos_infos(video_ids).await
    }
}

//////// END
