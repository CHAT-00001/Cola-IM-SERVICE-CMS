// /port/add.rs  -- 添加
// 2026/6/10 06:35

////////

use crate::video::command::video::edit::VideoUpdateCommand;
use crate::video::command::video::new::VideoNewCommand;
use crate::video::command::video::permission::VideoUpdatePermissionCommand;

////////

/// # [PORT] - 添加
#[async_trait::async_trait]
pub trait AddPort: Send + Sync {
    ////////

    /// # 1. [PORT] - 发布
    async fn add_video(&self, uid: i64, data: VideoNewCommand) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 编辑
    async fn edit_video(
        &self,
        uid: i64,
        video_id: i64,
        data: VideoUpdateCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 修改权限
    async fn change_permission(
        &self,
        uid: i64,
        _video_id: i64,
        data: VideoUpdatePermissionCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - 单个删除
    async fn del_one_video(&self, uid: i64, video_id: i64) -> anyhow::Result<()>;

    ////////

    /// # 5. [PORT] - 多个删除
    async fn del_many_video(&self, uid: i64, video_ids: Vec<i64>) -> anyhow::Result<()>;
}
