// port/src/video/hotlist/add.rs -- 端口 - VIDEO - 上热门 - 发布端口
// 2026/8/5 00:02 Created.

////////

use cola_data::cola_coc::command::hotlist::record::HotlistCommand;
use cola_data::cola_video::command::hotlist::VideoHotlistCommand;

////////

/// # [ADD PORTS] - 视频上热门发布端口
/// * `desc`: `VIDEO - Hotlist Add Port.`
#[async_trait::async_trait]
pub trait VideoHotlistAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 保存热门
    async fn save_hotlist(
        &self,
        uid: i64,                 // 操作者 ID
        video_id: i64,            // 视频 ID
        cmd: HotlistCommand, // 上热门命令
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 编辑热门
    async fn edit_hotlist(
        &self,
        uid: i64,        // 操作者 ID
        hotlist_id: i64, // 上热门 ID
    ) -> anyhow::Result<()>;
}

//////// END
