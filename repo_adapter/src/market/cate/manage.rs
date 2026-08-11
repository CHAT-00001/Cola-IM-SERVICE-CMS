// repo_adapter/src/market/cate/manage.rs
// 🔌 插头 - MARKET - 商品分类 - 管理
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::video::VideoInfo;
use port::market::cate::manage::CateManagePort;
////////

/// # [MANAGE ADAPTER] - 管理
/// * `desc`: `商品分类管理适配器`
pub struct CateManageAdapter;

#[async_trait]
impl CateManagePort for CateManageAdapter {
    //

    ////////

    /// # [ADAPTER] - 管理员列表
    async fn admin_get_videos_infos(
        &self,
        uid: i64,
        user_id: Option<i64>,
        video_id: Option<i64>,
        category_id: Option<i64>,
        channel_id: Option<i64>,
        keyword: Option<String>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        status_code: i16,
        limit: i64,
        offset: i64,
    ) -> Result<(VideoInfo), u64> {
        todo!()
    }
}

//////// END
