// repo_adapter/src/video/view/manage.rs
// 🔌 适配器 - VIDEO - 视频 - 浏览记录 - 管理
// 2026/8/8 12:00

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::view::VideoViewInfo;
use port::market::view::manage::GoodsViewManagePort;

////////

/// # [MANAGE ADAPTER] - 管理
/// * `desc`: `视频浏览管理服务适配器`
pub struct GoodsViewManageAdapter;

////////

#[async_trait]
impl GoodsViewManagePort for GoodsViewManageAdapter {
    //

    ////////

    /// # [ADAPTER] - 管理员列表
    async fn admin_get_views_infos(
        &self,
        uid: i64,
        user_id: Option<i64>,
        video_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        status_code: i16,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoViewInfo>), u64> {
        todo!()
    }
}

//////// END
