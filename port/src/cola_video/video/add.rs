// port/src/cola_video/video/add.rs
// ⏩️ 端口 - 可乐视频 -  视频 - 发布
// 2026/8/4 23:58 Created.

////////

use cola_data::cola_video::command::video::edit::VideoUpdateCommand;
use cola_data::cola_video::command::video::new::VideoNewCommand;
use cola_data::cola_video::command::video::permission::VideoUpdatePermissionCommand;

////////

/// # [ADD PORTS] - 发布
/// * `desc`: `▶ 可乐视频 - 视频发布端口`
#[async_trait::async_trait]
pub trait VideoAddPort: Send + Sync {
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

    /// # 2. [PORT] - 编辑
    /// * `desc`: `检查视频状态`
    async fn edit_video(
        &self,
        uid: i64,                 // UID
        video_id: i64,            // 视频ID
        data: VideoUpdateCommand, // 命令
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
