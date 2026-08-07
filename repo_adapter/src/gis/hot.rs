// repo_adapter/src/cola_gis/hot.rs
// 2026-07-07

////////

use async_trait::async_trait;
use cola_data::cola_gis::port::hot::HotlistRepo;
use cola_data::cola_gis::command::hotlist::HotlistCommand;
use repository::cola_gis::service::hotlist::HotlistService;

////////

/// # [HOTLIST PORT] - 上热门 端口 插头
pub struct HotlistPortAdapter;

#[async_trait]
impl HotlistRepo for HotlistPortAdapter {

    ////////

    /// # 1. [PORT] - 保存上热门记录
    async fn save_hotlist_record(
        &self,
        _uid: i64,
        _poi_id: i64,
        _cmd: HotlistCommand,
    ) -> anyhow::Result<()> {
        // TODO: implement with GIS hotlist service
        Ok(())
    }

    ////////

    /// # 2. [PORT] - 编辑热门记录
    async fn edit_hotlist_record(
        &self,
        _uid: i64,
        _poi_id: i64,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

//////// END