// port/src/cola_video/comment/list.rs
// ⏩️ 端口 - VIDEO - 评论 - 列表
// 2026/8/5 02:06 Created.

////////

use cola_data::cola_video::info::comment::VideoCommentInfo;

////////

/// # [LIST PORTS] - 列表
/// * `desc`: `VIDEO - 评论列表端口`
#[async_trait::async_trait]
pub trait VideoCommentListPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 我的
    /// * `desc`: `获取我的评论记录`
    async fn get_my_like_record(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(VideoCommentInfo)>;

    ////////

    /// # [PORT] - TA的
    /// * `desc`: `获取TA的评论记录`
    async fn get_he_like_record(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(VideoCommentInfo)>;
}

//////// END
