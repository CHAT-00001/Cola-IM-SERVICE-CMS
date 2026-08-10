// port/src/cola_video/danmaku/add.rs
// ⏩️ 端口 - 可乐视频 - 弹幕 - 发布
// 2026/8/5 00:05 Created.

////////

use cola_data::cola_video::command::danmaku::DanmakuCommand;
use cola_data::cola_video::info::danmaku::DanmakuInfo;

////////

/// # [ADD PORTS] - 发布
/// * `desc`: `▶ 可乐视频 - 弹幕发布端口`
#[async_trait::async_trait]
pub trait VideoDanmakuAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 发送
    /// * `desc`: `⏹ 可乐视频 - 根据视频ID发送弹幕`
    async fn send_danmaku(
        &self,
        uid: i64,            // UID
        video_id: i64,       // 视频 ID
        cmd: DanmakuCommand, // 命令
    ) -> anyhow::Result<(DanmakuInfo)>;

    ////////

    /// # 2. [PORT] - 更新
    /// * `desc`: `⏹ 可乐视频 - 根据弹幕ID 更新弹幕`
    async fn edit_danmaku(
        &self,
        uid: i64,            // UID
        danmaku_id: i64,     // 弹幕 ID
        cmd: DanmakuCommand, // 命令
    ) -> anyhow::Result<(DanmakuInfo)>;
}
