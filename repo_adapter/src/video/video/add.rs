// repo_adapter/src/cola_video/cola_video/add.rs
// 🔌 插头 - 可乐视频 - 视频 - 发布服务
// 2026-06-12

////////

use async_trait::async_trait;
use anyhow::Result;
use cola_data::cola_gis::port::add::AddPort;
use cola_data::cola_gis::command::poi::PoiCommand;

////////

/// # [ADD SERVICE] - 发布
/// * `desc`: `🔌 视频发布插头`
pub struct VideoAddAdapter;

#[async_trait]
impl AddPort for VideoAddAdapter {
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