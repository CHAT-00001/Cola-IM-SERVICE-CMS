// video/port/like/add.rs
// 视频 - port - 点赞 - 发布
// 2026/8/5 00:02 Created.

////////

use crate::video::command::hotlist::HotlistCommand;

////////

/// # [ADD PORTS] - 点赞
/// * `desc`: `视频点赞发布端口`
#[async_trait::async_trait]
pub trait LikeAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 保存
    async fn save_hotlist_record(
        &self,
        uid: i64,            // 操作者ID
        video_id: i64,       // 视频ID
        cmd: HotlistCommand, // 命令
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 编辑
    async fn edit_hotlist_record(
        &self,
        uid: i64,      // 操作者ID
        video_id: i64, // 视频ID
    ) -> anyhow::Result<()>;
}

//////// END
