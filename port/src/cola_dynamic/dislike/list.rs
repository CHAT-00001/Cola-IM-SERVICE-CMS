// port/src/cola_dynamic/dislike/list.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 不喜欢 - 列表
// 2026/8/5 16:00 Created.

////////

use cola_data::cola_video::info::comment::VideoCommentInfo;

////////

/// # [LIST PORTS] - 列表
/// * `desc`: `⏹ 可乐动态 - 动态不喜欢列表端口`
#[async_trait::async_trait]
pub trait DislikeListPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 动态的
    /// * `desc`: `获取我的评论记录`
    async fn get_dislike_infos_by_dynamic_id(
        &self,
        dynamic_id: i64, //动态 ID
        limit: i64,      // 数量
        offset: i64,     // 页码
    ) -> anyhow::Result<(VideoCommentInfo)>;

    ////////

    /// # [PORT] - 用户的
    /// * `desc`: `获取TA的评论记录`
    async fn get_dislike_infos_by_user_id(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(VideoCommentInfo)>;
}

//////// END
