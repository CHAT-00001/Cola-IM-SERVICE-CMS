// port/src/market/address/list.rs
// ⏩️ 端口 - MARKET - 地址簿 - 列表端口
// 2026/8/7 06:39 Created.

////////

use cola_data::cola_video::info::view::VideoViewInfo;

////////

/// # [LIST PORTS]
/// * `desc`: `MARKET - 地址簿列表端口`
#[async_trait::async_trait]
pub trait AddressListPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 用户的地址簿信息
    async fn get_address_infos_by_user_id(
        &self,
        user_id: i64, // 用户ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<VideoViewInfo>)>;

    ////////

    /// # 1. [PORT] - 地址簿的被动浏览记录
    async fn get_address_infos_by_video_id(
        &self,
        video_id: i64, // 用户ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> anyhow::Result<(Vec<VideoViewInfo>)>;
}

//////// END
