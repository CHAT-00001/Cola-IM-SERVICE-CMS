// port/src/cola_dynamic/hotlist/list.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 推荐 - 列表
// 2026/8/5 18:38 Created.

////////

use cola_data::cola_video::info::comment::VideoCommentInfo;

////////

/// # [LIST PORTS] - 列表
/// * `desc`: `⏹ 可乐动态 - 动态推荐列表端口`
#[async_trait::async_trait]
pub trait DynamicRecommendListPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 我的
    /// * `desc`: `获取我的评论记录`
    async fn get_my_recommend_infos(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(VideoCommentInfo)>;

    ////////

    /// # [PORT] - TA的
    /// * `desc`: `获取TA的评论记录`
    async fn get_he_recommend_infos(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(VideoCommentInfo)>;
}

//////// END
