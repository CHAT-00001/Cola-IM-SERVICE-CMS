// port/share.rs  -- 端口 分享
// 2026/6/10 08:17

////////

use crate::video::command::share::ShareCommand;

////////

/// # [SERVICE PORT] - 分享 服务端口
#[async_trait::async_trait]
pub trait ShareRepo: Send + Sync {

    ////////

    /// # [PORT] - 保存分享
    async fn save_share_record(
        &self,
        uid: i64,
        video_id: i64,
        cmd: ShareCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 删除
    async fn delete_share_record(
        &self,
        uid: i64,
        video_id: i64,
    ) -> anyhow::Result<()>;
}