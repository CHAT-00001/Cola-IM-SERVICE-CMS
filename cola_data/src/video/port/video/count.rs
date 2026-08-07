// video/port/video/count.rs
// 视频 - port - 视频 - 计数
// 2026/8/5 00:00 Created.

////////

use crate::video::command::video::edit::VideoUpdateCommand;
use crate::video::command::video::new::VideoNewCommand;
use crate::video::command::video::permission::VideoUpdatePermissionCommand;

////////

/// # [ADD PORTS] - 计数
/// * `desc`: `视频计数端口`
#[async_trait::async_trait]
pub trait VideoCountPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 发布
    /// * `desc`: `检查视频状态`
    async fn add_video(
        &self,
        uid: i64,              // UID
        data: VideoNewCommand, // 命令
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 更新
    /// * `desc`: `更新视频计数`
    async fn update_count(
        &self,
        uid: i64,      // UID
        video_id: i64, // 视频ID
    ) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 修改权限
    /// * `desc`: `用户修改视频权限`
    async fn change_permission(
        &self,
        uid: i64,       // UID
        _video_id: i64, // 视频ID
        data: VideoUpdatePermissionCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - 修改状态
    /// * `desc`: `用户修改视频权限`
    async fn change_state(
        &self,
        uid: i64,       // UID
        _video_id: i64, // 视频ID
        data: VideoUpdatePermissionCommand,
    ) -> anyhow::Result<()>;
}

//////// END
