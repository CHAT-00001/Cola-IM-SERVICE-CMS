// cola_video/port/danmaku/check.rs
// 视频 - port - 弹幕 - 检查
// 2026/8/5 13:35 Created.

////////

use crate::cola_video::command::danmaku::DanmakuCommand;
use crate::cola_video::info::danmaku::DanmakuInfo;

////////

/// # [CHECK SERVICE] - 检查
/// * `desc`: `弹幕检查服务`
#[async_trait::async_trait]
pub trait DanmakuCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 健康
    /// * `desc`: `检查目标健康`
    async fn health(
        &self,
        uid: i64,        // UID
        danmaku_id: i64, // 视频ID
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 状态
    /// * `desc`: `检查目标状态`
    async fn state(
        &self,
        uid: i64,        // UID
        danmaku_id: i64, // 弹幕ID
    ) -> anyhow::Result<()>;
}

//////// END
