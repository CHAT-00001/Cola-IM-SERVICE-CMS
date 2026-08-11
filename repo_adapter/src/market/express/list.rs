// repo_adapter/src/cola_video/cola_video/list.rs
// 🔌 插头 - 可乐视频 - 视频 - 列表
// 2026/8/7 05:31 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_market::info::express::express::ExpressInfo;
use cola_data::cola_video::info::video::VideoInfo;
use port::market::express::list::ExpressListPort;
use port::cola_video::video::list::VideoListPort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `🔌 视频发布插头`
pub struct ExpressListAdapter;

#[async_trait]
impl ExpressListPort for ExpressListAdapter {
    async fn get_view_infos_by_user_id(&self, user_id: i64, limit: i64, offset: i64) -> Result<(Vec<ExpressInfo>)> {
        todo!()
    }

    async fn get_new_infos(&self, limit: i64, offset: i64) -> Result<(Vec<ExpressInfo>)> {
        todo!()
    }

    async fn batch_get_infos_by_ids(&self, ids: Vec<i64>) -> Result<(Vec<ExpressInfo>)> {
        todo!()
    }
}

//////// END
