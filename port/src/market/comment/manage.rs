// port/src/market/comment/manage.rs
// ⏩️ 端口 - MARKET - 商品评论 - 管理
// 2026/8/5 15:23 Created.

////////

use cola_data::cola_video::info::comment::VideoCommentInfo;
use std::todo;

////////

/// # [MANAGE PORTS] - 管理
/// * `desc`: `MARKET - 商品评论管理端口`
#[async_trait::async_trait]
pub trait GoodsCommentManagePort: Send + Sync {
    //

    ////////

    /// # [PORT] - 管理员列表
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
    ) -> anyhow::Result<(Vec<VideoCommentInfo>)>;
}

//////// END
