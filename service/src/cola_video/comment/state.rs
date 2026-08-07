// servicey/src/cola_video/comment/state.rs
// 👤 服务 - ▶ 可乐视频  - 评论 - 状态
// 2026/8/2 17:20 Created.

////////

/// # [SERVICE] - 视频 评论 状态 服务
pub struct VideoCommentStateService;

impl VideoCommentStateService {
    //

    ////////

    /// # 1. [SERVICE] - 检查评论状态
    pub async fn check_comment_state(_uid: i64, comment_id: i64) -> Result<(), anyhow::Error> {
        // TODO: 购买付费视频/电商挂载商品落单逻辑

        Ok(())
    }

}

//////// END
