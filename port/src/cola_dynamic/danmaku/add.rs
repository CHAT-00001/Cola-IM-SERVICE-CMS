// port/src/cola_dynamic/danmaku/add.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 弹幕 - 发布
// 2026/8/5 00:05 Created.

////////

use cola_data::cola_video::command::danmaku::DanmakuCommand;
use cola_data::cola_video::info::danmaku::DanmakuInfo;

////////

/// # [ADD PORTS] - 发布
/// * `desc`: `⏹ 可乐动态 - 弹幕发布服务`
#[async_trait::async_trait]
pub trait DynamicDanmakuAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 发送
    /// * `desc`: `⏹ 可乐动态 - 根据动态ID发送弹幕`
    async fn send_danmaku(
        &self,
        uid: i64,            // UID
        dynamic_id: i64,     // 动态 ID
        cmd: DanmakuCommand, // 命令
    ) -> anyhow::Result<(DanmakuInfo)>;

    ////////

    /// # 2. [PORT] - 更新
    /// * `desc`: `⏹ 可乐动态 - 根据弹幕ID 更新弹幕`
    async fn edit_danmaku(
        &self,
        uid: i64,            // UID
        danmaku_id: i64,     // 弹幕 ID
        cmd: DanmakuCommand, // 命令
    ) -> anyhow::Result<(DanmakuInfo)>;
}

//////// END
