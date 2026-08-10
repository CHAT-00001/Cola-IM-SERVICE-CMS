// port/src/cola_dynamic/share/list.rs
// ⏩️ 端口 - 可乐动态 - 分享 - 获取
// 2026/8/5 15:35 Created.

////////

use cola_data::cola_dynamic::info::comment::DynamicCommentInfo;

////////

/// # [LIST SERVICE] - 列表
/// `desc`: `视频分享列表服务端口`
#[async_trait::async_trait]
pub trait DynamicShareListPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 动态的
    /// * `desc`: `⏹ 根据动态ID` - `获取分享记录信息`
    async fn get_share_infos_by_dynamic_id(
        &self,
        uid: i64,        // UID
        dynamic_id: i64, // 动态 ID
        limit: i64,      // 数量
        offset: i64,     // 页码
    ) -> anyhow::Result<(Vec<DynamicCommentInfo>)>;

    ////////

    /// # 2. [PORT] - 保存
    ///  * `desc`: `🗣 根据用户ID` - `获取分享记录信息`
    async fn get_share_infos_by_user_id(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<DynamicCommentInfo>)>;
}

//////// END
