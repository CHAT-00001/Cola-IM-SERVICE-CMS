// cola_video/port/cola_video/get.rs
// ⏩️ 端口 - 可乐视频 -  视频 - 获取
// 2026/8/5 00:00 Created.

////////

use cola_data::cola_market::info::shop::shop_apply::ShopInfo;

////////

/// # [GET PORTS] -  获取
/// * `desc`: `获取视频`
#[async_trait::async_trait]
pub trait ShopGetPort: Send + Sync {
    ////////

    /// # 1. [PORT] - 我的
    /// * `desc`: `▶ 可乐视频` - 获取我的视频列表信息
    /// * `condition`: `⚠️ 无视` - `无视 `权限/状态`
    async fn get_my_list(
        &self,
        uid: i64,                // UID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
        is_liked: bool,
    ) -> anyhow::Result<(Vec<ShopInfo>)>;

    ////////

    /// # 2. [PORT] - TA的
    /// * `desc`: `▶ 可乐视频` - 获取TA的视频列表信息
    /// * `condition`: `⚠️ 受限` - `权限/状态`
    async fn get_he_list(
        &self,
        uid: i64,                // UID
        user_id: i64,            // 用户 ID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(Vec<ShopInfo>)>;
}

//////// END
