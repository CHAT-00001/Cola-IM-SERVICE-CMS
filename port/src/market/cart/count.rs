// port/src/market/cart/count.rs
// ⏩️ 端口 - MARKET - CART - 计数
// 2026/8/5 00:00 Created.

////////

use cola_data::cola_video::command::video::new::VideoNewCommand;
use cola_data::cola_video::info::video::VideoInfo;

////////

/// # [COUNT PORTS] - 计数
/// * `desc`: `购物车计数端口`
#[async_trait::async_trait]
pub trait CartCountPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 发布
    /// * `desc`: `添加计数`
    async fn add_count(
        &self,
        video_id: i64,         // 视频 ID
        data: VideoNewCommand, // 命令
    ) -> anyhow::Result<(VideoInfo)>;

    ////////

    /// # 2. [PORT] - 更新
    /// * `desc`: `更新视频计数`
    async fn update_count(
        &self,
        video_id: i64,           // 视频 ID
        views: Option<i64>,      // 浏览数量
        comments: Option<i64>,   // 评论数量
        danmakus: Option<i64>,   // 弹幕数量
        collects: Option<i64>,   // 收藏数量
        recommends: Option<i64>, // 推荐数量
        shares: Option<i64>,     // 分享数量
    ) -> anyhow::Result<(VideoInfo)>;

    ////////

    /// # 3. [PORT] - 获取
    /// * `desc`: `▶ AUTO` - 自动获取视频计数
    async fn get_count(
        &self,
        video_id: i64, // 视频 ID
    ) -> anyhow::Result<(VideoInfo)>;

    ////////

    /// # 4. [PORT] - 删除
    /// * `desc`: `▶ AUTO` - 自动删除视频计数
    async fn delete_count(
        &self,
        uid: i64,      // UID 审计使用
        video_id: i64, // 视频 ID
    ) -> anyhow::Result<(u64)>;
}

//////// END
