// repo_adapter/src/cola_gis/ticket
// 🔌 适配器 - 可乐GIS - POI - 分享
// 2026-07-07

//////

use async_trait::async_trait;
use cola_data::cola_gis::command::share::ShareCommand;
use port::cola_gis::share::ShareRepo;

//////

/// # [ADAPTER] - POI 分享
/// * `desc`: `GIS - POI分享`
pub struct SharePortAdapter;

//////

#[async_trait]
impl ShareRepo for SharePortAdapter {
    //

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
    async fn delete_share_record(&self, _uid: i64, _poi_id: i64) -> anyhow::Result<()> {
        Ok(())
    }
}

////// END
