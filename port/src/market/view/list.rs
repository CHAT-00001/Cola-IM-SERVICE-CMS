// port/src/cola_video/view/list.rs
// ⏩️ 端口 - ▶ 可乐视频 - 浏览 - 列表端口
// 2026/8/7 06:39 Created.

////////

use cola_data::cola_market::info::goods::view::GoodsViewInfo;

////////

/// # [LIST PORTS]
/// * `desc`: `视频浏览列表端口`
#[async_trait::async_trait]
pub trait GoodsViewListPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 用户的主动浏览记录
    async fn get_view_infos_by_user_id(
        &self,
        user_id: i64, // 用户ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<GoodsViewInfo>)>;

    ////////

    /// # 1. [PORT] - 视频的被动浏览记录
    async fn get_view_infos_by_video_id(
        &self,
        video_id: i64, // 用户ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> anyhow::Result<(Vec<GoodsViewInfo>)>;
}

//////// END
