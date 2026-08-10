// port/src/cola_dynamic/dynamic/count.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 动态 - 计数
// 2026/8/5 00:00 Created.

////////

use cola_data::cola_dynamic::command::dynamic::DynamicCommand;
use cola_data::cola_dynamic::info::dynamic::DynamicInfo;

////////

/// # [COUNT PORTS] - 计数
/// * `desc`: `⏹ 可乐动态 - 动态数量端口`
#[async_trait::async_trait]
pub trait CountPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 添加
    /// * `desc`: `⏹ 可乐动态 - 添加动态技术`
    async fn add_count(
        &self,
        dynamic_id: i64,       // 动态 ID
        count: DynamicCommand, // 命令
    ) -> anyhow::Result<(DynamicInfo)>;

    ////////

    /// # 2. [PORT] - 更新
    /// * `desc`: `⏹ 可乐动态 - 更新视频计数`
    async fn update_count(
        &self,
        dynamic_id: i64,         // 动态 ID
        views: Option<i64>,      // 浏览数量
        likes: Option<i64>,      // 点赞数量
        comments: Option<i64>,   // 评论数量
        danmakus: Option<i64>,   // 弹幕数量
        recommends: Option<i64>, // 推荐数量
        collects: Option<i64>,   // 收藏数量
        shares: Option<i64>,     // 分享数量
    ) -> anyhow::Result<()>;
}

//////// END
