// video/port/share/add.rs
// 视频 - port - 分享 - 发布
// 2026/8/5 00:00 Created.

////////

use crate::video::command::share::ShareCommand;

////////

/// # [SHARE PORTS] - 分享
/// `desc`: `视频分享发布服务端口`
#[async_trait::async_trait]
pub trait AddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 保存
    async fn save_share_record(
        &self,
        uid: i64,          // UID
        video_id: i64,     // 视频ID
        cmd: ShareCommand, // 命令
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 删除
    async fn delete_share_record(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频ID
    ) -> anyhow::Result<()>;
}

//////// END
