// video/port/video/check.rs
// 视频 - port - 视频 - 检查
// 2026/8/5 00:00 Created.

////////

use crate::video::info::video::VideoInfo;

////////

/// # [CHECK PORTS] - 管理
/// * `desc`: `视频检查端口`
#[async_trait::async_trait]
pub trait VideoCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 检查健康
    /// * `desc`: `检查视频健康`
    async fn check_health(
        &self,
        uid: i64,                // UID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
        is_liked: bool,
    ) -> anyhow::Result<(Vec<VideoInfo>)>;

    ////////

    /// # 2. [PORT] - 检查状态
    /// * `desc`: `检查视频状态`
    async fn check_state(
        &self,
        uid: i64,                // UID
        keyword: Option<String>, // 关键词
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(Vec<VideoInfo>)>;
}

//////// END
