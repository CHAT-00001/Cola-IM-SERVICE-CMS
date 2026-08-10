// prot/src/cola_video/danmaku/stat.rs
// ⏩️ 端口 - ▶ 可乐视频 - port - 弹幕 - 统计
// 2026/8/5 00:06 Created.

////////

use cola_data::cola_video::command::danmaku::DanmakuCommand;
use cola_data::cola_video::info::danmaku::DanmakuInfo;

////////

/// # [STAT PORTS] - 统计
/// * `desc`: `▶ 可乐视频 - 弹幕统计端口`
#[async_trait::async_trait]
pub trait VideoDanmakuStatPort: Send + Sync {
    //

    ////////

    ////////

    /// # [PORT] - 用户的
    /// * `desc`: `根据用户ID` - `统计购买数量`
    async fn stat_count_by_user_id(
        &self,
        uid: i64,
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<(u64)>;

    ////////

    /// # [PORT] - 视频的
    /// * `desc`: `根据视频ID` - `统计购买数量`
    async fn stat_count_by_video_id(
        &self,
        uid: i64,
        video_id: i64, // 视频 ID
    ) -> anyhow::Result<(u64)>;
}

//////// END
