// port/src/cola_dynamic/hotlist/add.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 上热门 - 发布
// 2026/8/5 00:02 Created.

////////

use cola_data::cola_gis::command::hotlist::HotlistCommand;

////////

/// # [ADD PORTS] - 上热门
/// * `desc`: `⏹ 可乐动态 - 上热门发布端口`
#[async_trait::async_trait]
pub trait HotlistAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 上热门
    async fn send_hotlis(
        &self,
        uid: i64,            // 操作者ID
        video_id: i64,       // 视频ID
        cmd: HotlistCommand, // 命令
    ) -> anyhow::Result<(bool)>;
}

//////// END
