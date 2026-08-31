// repo_adapter/src/market/comment/manage.rs
// 🔌 适配器 - MARKET - 商品评论 - 管理
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::comment::VideoCommentInfo;
use port::market::comment::manage::GoodsCommentManagePort;

////////

/// # [MANAGE ADAPTER] - 管理
/// * `desc`: `VIDEO - 视频评论管理适配器`
#[derive(Debug, Default, Clone)]
pub struct GoodsCommentManageAdapter;

#[async_trait]
impl GoodsCommentManagePort for GoodsCommentManageAdapter {
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
    ) -> anyhow::Result<(Vec<VideoCommentInfo>)> {
        todo!()
    }
}

//////// END
