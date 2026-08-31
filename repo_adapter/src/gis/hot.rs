// repo_adapter/src/cola_gis/hot.rs -- 🔌 适配器 - 可乐GIS - POI - 热门
// 2026-07-07 18:41

////////

use async_trait::async_trait;
use cola_data::cola_gis::command::hotlist::{PoiHotlistCommand};
use port::cola_gis::hot::HotlistRepo;

////////

/// # [HOTLIST ADAPTER] - 上热门 端口 插头
/// * `desc`: `GIS - POI 上热门适配器`
pub struct HotlistPortAdapter;

#[async_trait]
impl HotlistRepo for HotlistPortAdapter {
    //

    ////////

    /// # 1. [PORT] - 保存上热门记录
    async fn save_hotlist_record(
        &self,
        _uid: i64,
        _poi_id: i64,
        _cmd: PoiHotlistCommand,
    ) -> anyhow::Result<()> {
        // TODO: implement with GIS hotlist service
        Ok(())
    }

    ////////

    /// # 2. [PORT] - 编辑热门记录
    async fn edit_hotlist_record(&self, _uid: i64, _poi_id: i64) -> anyhow::Result<()> {
        Ok(())
    }
}

//////// END
