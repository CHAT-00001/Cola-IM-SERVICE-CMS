// repo_adapter/src/market/address/list.rs
// 🔌 插头 - MARKET - ADDRESS - 列表
// 2026/8/7 05:31 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::view::VideoViewInfo;
use port::market::address::list::AddressListPort;

////////

/// # [LIST ADAPTER] - 列表
/// * `desc`: `MARKET - 地址列表适配器`
pub struct AddressListAdapter;

#[async_trait]
impl AddressListPort for AddressListAdapter {
    /// # 1. [ADAPTER] - 用户的
    async fn get_address_infos_by_user_id(
        &self,
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoViewInfo>)> {
        todo!()
    }

    async fn get_address_infos_by_video_id(
        &self,
        video_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<VideoViewInfo>)> {
        todo!()
    }
}

//////// END
