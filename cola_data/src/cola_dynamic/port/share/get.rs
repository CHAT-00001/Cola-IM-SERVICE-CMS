// cola_video/port/share/get.rs
// 视频 - port - 分享 - 获取
// 2026/8/5 00:00 Created.

////////

use crate::cola_video::command::share::ShareCommand;

////////

/// # [DEL SERVICE] - 获取
/// `desc`: `视频分享获取服务端口`
#[async_trait::async_trait]
pub trait GetPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个软删除
    /// * `desc`: `用户单个软删除分享记录`
    async fn single_soft_del_record(
        &self,
        uid: i64,          // UID
        video_id: i64,     // 视频ID
        id: i64,           // 目标ID
        cmd: ShareCommand, // 命令
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 保存
    /// * `desc`: `用户批量软删除分享记录`
    async fn batch_soft_del_record(
        &self,
        uid: i64,          // UID
        video_id: i64,     // 视频ID
        ids: Vec<i64>,     // 目标IDs
        cmd: ShareCommand, // 命令
    ) -> anyhow::Result<(u16)>;
}

//////// END
