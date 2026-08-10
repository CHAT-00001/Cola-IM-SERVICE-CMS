// port/src/cola_video/danmaku/alive.rs
// ⏩️ 端口 - ▶ 可乐视频 - 弹幕 - 存活
// 2026/8/5 00:05 Created.

////////

use cola_data::cola_video::command::danmaku::DanmakuCommand;
use cola_data::cola_video::info::danmaku::DanmakuInfo;


////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `弹幕发布服务`
#[async_trait::async_trait]
pub trait VideoDanmakuAlivePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 保存
    /// * `desc`: `保存弹幕记录`
    async fn save_danmaku_record(
        &self,
        uid: i64,            // UID
        video_id: i64,       // 视频ID
        cmd: DanmakuCommand, // 命令
    ) -> anyhow::Result<(DanmakuInfo)>;
}
