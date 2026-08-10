// port/src/cola_dynamic/port/share/get.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 分享 - 获取
// 2026/8/5 00:00 Created.

////////

use cola_data::cola_dynamic::info::comment::DynamicCommentInfo;

////////

/// # [GET PORTS] - 获取
/// `desc`: `⏹ 可乐动态 - 分享获取端口`
#[async_trait::async_trait]
pub trait DynamicShareGetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 我的
    /// * `desc`: `🗣 USER` - `根据UID` - `我的分享记录信息`
    async fn get_my_share_infos(
        &self,
        uid: i64,     // UID
        user_id: i64, // 视频ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<Vec<DynamicCommentInfo>>;

    ////////

    /// # 1. [PORT] - 我的
    /// * `desc`: `🗣 USER` - `根据UID` - `我的分享记录信息`
    async fn get_he_share_infos(
        &self,
        uid: i64,     // UID
        user_id: i64, // 视频ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<Vec<DynamicCommentInfo>>;
}

//////// END
