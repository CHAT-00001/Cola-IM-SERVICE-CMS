// port/add  -- 端口 分享
// 2026/7/7

//////

use crate::gis::command::share::ShareCommand;

//////

/// # [SERVICE PORT] - 分享 服务端口
#[async_trait::async_trait]
pub trait ShareRepo: Send + Sync {

    ////////

    /// # [PORT] - 保存分享记录
    async fn save_share_record(
        &self,
        uid: i64,
        poi_id: i64,
        cmd: ShareCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # [PORT] - 删除分享记录
    async fn delete_share_record(
        &self,
        uid: i64,
        poi_id: i64,
    ) -> anyhow::Result<()>;
}