// port/src/video/hotlist/add.rs
// ⏩️  端口 - ▶ 视频 - 上热门 - 发布
// 2026/8/5 00:02 Created.

////////



////////

use cola_data::cola_video::command::hotlist::HotlistCommand;

/// # [ADD PORTS] - 上热门
/// * `desc`: `▶ 视频 - 上热门发布端口`
#[async_trait::async_trait]
pub trait VideoHotlistAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 保存
    async fn save_hotlist(
        &self,
        uid: i64,            // 操作者 ID
        video_id: i64,       // 视频 ID
        cmd: HotlistCommand, // 命令
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 编辑
    async fn edit_hotlist(
        &self,
        uid: i64,        // 操作者 ID
        hotlist_id: i64, // 上热门 ID
    ) -> anyhow::Result<()>;
}

//////// END
