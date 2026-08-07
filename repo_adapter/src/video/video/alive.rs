// repo_adapter/src/video/video/alive.rs
// 🔌 插头 - 可乐视频 - 视频 - 存活
// 2026/8/6 19:20 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::gis::command::poi::PoiCommand;
use cola_data::gis::port::add::AddPort;

////////

/// # [ADD ADAPTER] - 存活(预设)
/// * `desc`: `🔌 视频存活服务`
pub struct VideoAliveAdapter;

#[async_trait]
impl AddPort for VideoAliveAdapter {
    async fn add_poi(&self, _uid: i64, _data: PoiCommand) -> Result<()> {
        Ok(())
    }
    async fn edit_poi(&self, _uid: i64, _poi_id: i64, _data: PoiCommand) -> Result<()> {
        Ok(())
    }
    async fn del_one_poi(&self, _uid: i64, _poi_id: i64) -> Result<()> {
        Ok(())
    }
    async fn del_many_poi(&self, _uid: i64, _poi_ids: Vec<i64>) -> Result<()> {
        Ok(())
    }
}

//////// END
