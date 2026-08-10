// port/src/cola_video/view/list.rs
// ⏩️ 端口 - ▶ 可乐视频 - 浏览 - 列表端口
// 2026/8/7 06:39 Created.

////////

use cola_data::cola_video::info::view::VideoViewInfo;

////////

/// # [LIST PORTS]
/// * `desc`: `视频浏览列表端口`
#[async_trait::async_trait]
pub trait VideoViewListPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 列表
    /// * `desc`: `浏览记录列表`
    async fn get_view_list(
        &self,
        user_id: i64, // 用户ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<VideoViewInfo>)>;
}

//////// END
