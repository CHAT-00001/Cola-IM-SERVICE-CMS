// cola_video/port/cola_video/del.rs
// 视频 - port - 视频 - 发布
// 2026/8/5 00:00 Created.

////////

use crate::cola_video::command::video::edit::VideoUpdateCommand;
use crate::cola_video::command::video::new::VideoNewCommand;
use crate::cola_video::command::video::permission::VideoUpdatePermissionCommand;

////////

/// # [DEL SERVICE] - 添加
/// * `desc`: `视频删除端口`
#[async_trait::async_trait]
pub trait VideoDelPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个软删除
    /// * `desc`: `用户批量删除视频`
    async fn single_soft_del(
        &self,
        uid: i64, // UID
        id: i64,  // 视频ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量软删除
    /// * `desc`: `用户批量删除视频`
    async fn batch_soft_del(
        &self,
        uid: i64,      // UID
        ids: Vec<i64>, // 视频IDs
    ) -> anyhow::Result<(u16)>;
}

//////// END
