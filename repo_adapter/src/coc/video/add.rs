// repo_adapter/src/video/cola_video/add.rs
// 🔌 插头 - VIDEO - 视频 - 发布服务
// 2026-06-12 10:52 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::command::video::edit::VideoUpdateCommand;
use cola_data::cola_video::command::video::new::VideoNewCommand;
use cola_data::cola_video::command::video::permission::VideoUpdatePermissionCommand;
use port::cola_video::video::add::VideoAddPort;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `🔌 视频发布插头`
pub struct VideoAddAdapter;

#[async_trait]
impl VideoAddPort for VideoAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 发布新视频
    async fn add_video(
        &self,
        uid: i64,              // 操作者 ID
        data: VideoNewCommand, // 视频发布命令
    ) -> Result<()> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 编辑视频
    async fn edit_video(
        &self,
        uid: i64,                 // 操作者 ID
        video_id: i64,            // 视频 ID
        data: VideoUpdateCommand, // 视频更新命令
    ) -> Result<()> {
        todo!()
    }

    ////////

    /// # 3. [ADAPTER] - 修改权限
    async fn change_permission(
        &self,
        uid: i64,                           // 操作者 ID
        _video_id: i64,                     // 视频 ID
        data: VideoUpdatePermissionCommand, // 视频风险权限命令
    ) -> Result<()> {
        todo!()
    }

    ////////

    /// # 4. [ADAPTER] - 修改状态
    async fn change_state(
        &self,
        uid: i64,                           // 操作者 ID
        _video_id: i64,                     // 视频 ID
        data: VideoUpdatePermissionCommand, // 视频更新权限命令
    ) -> Result<()> {
        todo!()
    }
}

//////// END
