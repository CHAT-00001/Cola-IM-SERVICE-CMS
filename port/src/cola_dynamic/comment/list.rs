// port/src/cola_dynamict/comment/list.rs
// ⏩️ 端口 - 可乐动态 - 评论 - 列表
// 2026/8/5 02:06 Created.

////////

use cola_data::cola_dynamic::info::comment::DynamicCommentInfo;

////////

/// # [LIST PORTS] - 列表
/// * `desc`: `⏹ 可乐动态 - 评论列表服务端口`
#[async_trait::async_trait]
pub trait DynamicCommentListPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 动态的
    /// * `desc`: `根据动态ID - 获取评论记录`
    async fn get_comments_list_by_video_id(
        &self,
        uid: i64,        // UID
        dynamic_id: i64, // 动态 ID
        limit: i64,      // 数量
        offset: i64,     // 页码
    ) -> anyhow::Result<(DynamicCommentInfo)>;

    ////////

    /// # 2. [PORT] - 用户的
    /// * `desc`: `根据用户ID - 获取评论记录`
    async fn get_comments_list_by_user_id(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(DynamicCommentInfo)>;

    ////////

    /// # 3. [PORT] - 最新
    /// * `desc`: `根据用户ID - 获取评论记录`
    async fn get_new_list(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(DynamicCommentInfo)>;

    ////////

    /// # 4. [PORT] - 热门
    /// * `desc`: `根据点赞/回复数量 - 获取评论记录`
    async fn get_hot_list(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(DynamicCommentInfo)>;

    ////////

    /// # 5. [PORT] - 同城
    /// * `desc`: `根据城市ID - 获取评论记录`
    async fn get_city_list(
        &self,
        uid: i64,     // UID
        city_id: i64, // 城市 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(DynamicCommentInfo)>;

    ////////

    /// # 6. [PORT] - 附近
    /// * `desc`: `根据用户ID - 获取评论记录`
    async fn get_nearby_list(
        &self,
        uid: i64,    // UID
        lat: f64,    // 纬度
        lng: f64,    // 经度
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(DynamicCommentInfo)>;
}

//////// END
