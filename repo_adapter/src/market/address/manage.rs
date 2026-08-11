// repo_adapter/src/market/addresso/manage.rs
// 🔌 适配器 - MARKET - ADDRESS - 管理
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::view::VideoViewInfo;
use port::market::address::manage::AddressManagePort;

////////

/// # [MANAGE ADAPTER] - 管理
/// * `desc`: `MARKET - 地址管理适配器`
pub struct AddressManageAdapter;

#[async_trait]
impl AddressManagePort for AddressManageAdapter {
    //

    ////////

    /// # [ADAPTER] - 管理员列表
    async fn admin_list(
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
