// repo_adapter/src/new/share.rs
// 2026-06-12

////////

use async_trait::async_trait;
use cola_data::video::port::share::ShareRepo;
use cola_data::video::command::share::ShareCommand;
use repository::video::service::danmaku::DanmakuService;

////////

pub struct SharePortAdapter;

////////

#[async_trait]
impl ShareRepo for SharePortAdapter {

    ////////

    /// # 1. [PORT] - 保存分享记录 + 更新分享数量
    async fn save_share_record(
        &self,
        uid: i64,
        video_id: i64,
        cmd: ShareCommand,
    ) -> anyhow::Result<()> {
        DanmakuService::save_share_and_update_count(uid, video_id, cmd).await
    }
    ////////

    /// # 2. [PORT] - 删除分享记录 + 更新分享数量
    /// * `DESC`: 预设,不开放,分享不可逆
    async fn delete_share_record(
        &self,
        _uid: i64,
        _video_id: i64,
    ) -> anyhow::Result<()> {
        // TODO: implement actual delete
        Ok(())
    }
}

//////// END