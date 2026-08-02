// repo_adapter/src/new/hot.rs
// 2026-06-12 09:30

////////

use async_trait::async_trait;
use cola_data::video::port::hot::HotlistRepo;
use cola_data::video::command::hotlist::HotlistCommand;
use repository::video::service::hotlist::HotlistService;

////////

/// # [HOTLIST PORT] - 上热门 端口 插头
pub struct HotlistPortAdapter;

#[async_trait]
impl HotlistRepo for HotlistPortAdapter {

    ////////

    /// # 1. [PORT] - 保存上热门记录 + 更新抖加数量
    async fn save_hotlist_record(
        &self,
        uid: i64,
        _video_id: i64,
        cmd: HotlistCommand,
    ) -> anyhow::Result<()> {
        HotlistService::save_hotlist_and_update_count(uid, cmd).await
    }

    ////////

    /// # 2. [PORT] - 更新抖加记录(预设)
    async fn edit_hotlist_record(
        &self,
        uid: i64,
        _video_id: i64,
    ) -> anyhow::Result<()> {
        // No existing edit; just a no-op placeholder
        Ok(())
    }
}

//////// END