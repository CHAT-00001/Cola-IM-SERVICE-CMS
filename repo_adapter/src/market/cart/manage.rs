// repo_adapter/src/market/cart/manage.rs
// 🔌 适配器 - MARKET - CART - 管理
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::video::VideoInfo;
use port::market::cart::manage::CartManagePort;

////////

/// # [MANAGE ADAPTER] - 管理
/// * `desc`: `购物车管理适配器`
pub struct CartManageAdapter;

#[async_trait]
impl CartManagePort for CartManageAdapter {
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
