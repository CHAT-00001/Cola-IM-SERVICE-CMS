// port/cola_dynamic/dynamic/add.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 动态 - 发布
// 2026/8/4 23:58 Created.

////////

use cola_data::cola_dynamic::command::dynamic::DynamicCommand;
use cola_data::cola_dynamic::info::dynamic::DynamicInfo;
use cola_data::cola_video::command::video::permission::VideoUpdatePermissionCommand;

////////

/// # [ADD PORTS] - 发布
/// * `desc`: `⏹ 可乐动态 - 动态发布端口`
#[async_trait::async_trait]
pub trait AddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 发布
    /// * `desc`: `⏹ 可乐动态 - ⏩️ 用户发布动态`
    async fn add_dynamic(
        &self,
        uid: i64,             // UID
        data: DynamicCommand, // 命令
    ) -> anyhow::Result<(DynamicInfo)>;

    ////////

    /// # 2. [PORT] - 编辑
    /// * `desc`: `⏹ 可乐动态 - ⏩️ 用户编辑动态`
    async fn edit_dynamic(
        &self,
        uid: i64,             // UID
        video_id: i64,        // 视频ID
        data: DynamicCommand, // 命令
    ) -> anyhow::Result<(DynamicInfo)>;

    ////////

    /// # 3. [PORT] - 修改权限
    /// * `desc`: `⏹ 可乐动态 - 用户修改视频权限`
    async fn change_permission(
        &self,
        uid: i64,        // UID
        dynamic_id: i64, // 动态 ID
        data: VideoUpdatePermissionCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - 修改状态
    /// * `desc`: `⏹ 可乐动态 - 用户修改视频权限`
    async fn change_state(
        &self,
        uid: i64,        // UID
        dynamic_id: i64, // 动态 ID
        data: VideoUpdatePermissionCommand,
    ) -> anyhow::Result<()>;
}

//////// END
