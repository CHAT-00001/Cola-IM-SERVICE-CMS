// repo_adapter/src/video/view/manage.rs  -- 适配器 - 视频浏览管理
// 2026/8/8 12:00

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::port::view::manage::VideoViewManagePort;
use cola_data::cola_video::info::video::VideoInfo;
use cola_data::cola_video::info::view::VideoViewInfo;

////////

/// # [MANAGE SERVICE] - 管理
/// * `desc`: `视频浏览管理服务适配器`
pub struct ViewManageService;

////////

#[async_trait]
impl VideoViewManagePort for ViewManageService {
    async fn save_view_record_update_views_count(
        &self,
        _uid: i64,       // 用户ID
        _video_id: i64,  // 视频ID
    ) -> Result<()> {
        Ok(())
    }

    async fn view_done_update_done_count(
        &self,
        _uid: i64,       // 用户ID
        _video_id: i64,  // 视频ID
        _is_done: bool,  // 是否完播
    ) -> Result<()> {
        Ok(())
    }

    async fn single_del_view_record_by_id(
        &self,
        _id: i64,        // 记录ID
    ) -> Result<u16> {
        Ok(0)
    }

    async fn batch_del_view_record_by_ids(
        &self,
        _ids: Vec<i64>,  // 记录IDs
    ) -> Result<u16> {
        Ok(0)
    }

    async fn get_video_list_by_ids(
        &self,
        _video_ids: Vec<i64>, // 视频IDs
    ) -> Result<Vec<VideoInfo>> {
        Ok(vec![])
    }

    async fn get_my_viewed_list(
        &self,
        _uid: i64,    // 用户ID
        _limit: i64,  // 数量
        _offset: i64, // 页码
    ) -> Result<Vec<VideoInfo>> {
        Ok(vec![])
    }

    async fn get_here_viewed_list(
        &self,
        _uid: i64,    // 用户ID
        _limit: i64,  // 数量
        _offset: i64, // 页码
    ) -> Result<Vec<VideoInfo>> {
        Ok(vec![])
    }

    async fn get_video_viewed_list(
        &self,
        _video_id: i64, // 视频ID
        _limit: i64,    // 数量
        _offset: i64,   // 页码
    ) -> Result<Vec<VideoViewInfo>> {
        Ok(vec![])
    }
}

//////// END
