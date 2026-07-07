// repo_adapter/src/gis/share.rs
// 2026-07-07

//////

use async_trait::async_trait;
use cola_data::gis::port::share::ShareRepo;
use cola_data::gis::command::share::ShareCommand;

//////

pub struct SharePortAdapter;

//////

#[async_trait]
impl ShareRepo for SharePortAdapter {

    ////////

    /// # 1. [PORT] - 保存分享记录
    async fn save_share_record(
        &self,
        _uid: i64,
        _poi_id: i64,
        _cmd: ShareCommand,
    ) -> anyhow::Result<()> {
        // TODO: implement with GIS share service
        Ok(())
    }

    ////////

    /// # 2. [PORT] - 删除分享记录
    async fn delete_share_record(
        &self,
        _uid: i64,
        _poi_id: i64,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

////// END