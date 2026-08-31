// port/src/gis/poi/hotlist  -- 端口 - GIS - 上热门 - mod
// 2026/7/7 14:10 Created.

////////

use cola_data::cola_gis::command::hotlist::PoiHotlistCommand;

////////

/// # [SERVICE] - 上热门
#[async_trait::async_trait]
pub trait HotlistRepo: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 保存
    async fn save_hotlist_record(
        &self,
        uid: i64,               // 操作者 ID
        poi_id: i64,            // POI ID
        cmd: PoiHotlistCommand, // 上热门命令
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 编辑
    async fn edit_hotlist_record(
        &self,
        uid: i64,    // 操作者 ID
        poi_id: i64, // POI ID
    ) -> anyhow::Result<()>;
}
