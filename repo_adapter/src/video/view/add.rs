// repo_adapter/src/cola_video/view/add.rs
// 🔌 插头 - 可乐视频 - 浏览 - 发布
// 2026/8/6 19:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::command::video::edit::VideoUpdateCommand;
use cola_data::cola_video::command::video::new::VideoNewCommand;
use cola_data::cola_video::command::video::permission::VideoUpdatePermissionCommand;
use cola_data::cola_video::port::video::add::VideoAddPort;
////////

/// # [DEL SERVICE] - 发布
/// * `desc`: `视频浏览发布服务`
pub struct AddService;

// 构造实现
#[async_trait]
impl VideoAddPort for AddService {
    //

    ////////

    /// # 1. [SERVICE] - 保存
    /// * `desc`: `发布视频`
    async fn add_video(&self, uid: i64, data: VideoNewCommand) -> Result<()> {
        todo!()
    }

    async fn edit_video(&self, uid: i64, video_id: i64, data: VideoUpdateCommand) -> Result<()> {
        todo!()
    }

    async fn change_permission(
        &self,
        uid: i64,
        _video_id: i64,
        data: VideoUpdatePermissionCommand,
    ) -> Result<()> {
        todo!()
    }

    async fn change_state(
        &self,
        uid: i64,
        _video_id: i64,
        data: VideoUpdatePermissionCommand,
    ) -> Result<()> {
        todo!()
    }
}

//////// END
