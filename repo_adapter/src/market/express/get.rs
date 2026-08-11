// repo_adapter/src/cola_video/cola_video/get.rs
// 🔌 插头 - 可乐视频 - 视频 - 获取IDs
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_video::info::video::VideoInfo;
use port::market::express::get::ExpressGetPort;
use port::cola_video::video::get::VideoGetPort;
////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `🔌 视频发布插头`
pub struct ExpressGetAdapter;

// 构造实现
#[async_trait]
impl ExpressGetPort for ExpressGetAdapter {
    async fn get_express_ids_by_user_id(&self, user_id: i64, limit: i64, offset: i64) -> Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
