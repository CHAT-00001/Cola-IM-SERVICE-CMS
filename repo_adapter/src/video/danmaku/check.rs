// repo_adapter/src/cola_video/danmaku/check.rs
// 🔌 适配器 - VIDEO - 弹幕 - 检查
// 2026/8/6 19:10 Created.

////////

use async_trait::async_trait;
use port::cola_video::danmaku::check::VideoDanmakuCheckPort;
////////

/// # [CHECK ADAPTER] - danmaku check
/// * `desc`: `VIDEO - 视频弹幕检查适配器`
#[derive(Debug, Default, Clone)]
pub struct VideDanmakuCheckAdapter;

#[async_trait]
impl VideoDanmakuCheckPort for VideDanmakuCheckAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 健康
    async fn check_health(
        &self,
        uid: i64,
        danmaku_id: i64, // 弹幕 ID
    ) -> anyhow::Result<(bool)> {
        todo!()
    }

    ////////

    /// # 1. [ADAPTER] - 健康
    async fn check_state(
        &self,
        uid: i64,
        danmaku_id: i64, // 弹幕 ID
    ) -> anyhow::Result<(bool)> {
        todo!()
    }

    ////////

    /// # 1. [ADAPTER] - 归属
    async fn is_owner(
        &self,
        uid: i64,
        user_id: i64,    // 用户 ID
        danmaku_id: i64, // 弹幕 ID
    ) -> anyhow::Result<(bool)> {
        todo!()
    }
}

//////// END
