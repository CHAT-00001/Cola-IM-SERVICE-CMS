// port/srcvideo/buy/list.rs
// ⏩️ 端口 - ▶ 可乐视频 - 购买 - 列表
// 2026/8/5 00:06 Created.

////////

use cola_data::cola_video::info::buy::VideoBuyInfo;

////////

/// # [LIST SERVICE] - 列表
/// * `desc`: `获取视频购买列表端口`
#[async_trait::async_trait]
pub trait VideoBuyListPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 用户的
    /// * `desc`: `根据用户ID` - `获取购买记录信息`
    async fn get_buy_infos_user_id(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<VideoBuyInfo>)>;

    ////////

    /// # [PORT] - 用户的
    /// * `desc`: `根据视频ID` - `获取购买记录信息`
    async fn get_buy_infos_video_id(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频 ID
        limit: i64,    // 数量
        offset: i64,   // 页码
    ) -> anyhow::Result<(Vec<VideoBuyInfo>)>;

    ////////
}

//////// END
