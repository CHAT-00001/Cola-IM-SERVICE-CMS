// repo_adapter/src/cola_video/hotlist/get.rs
// 🔌 插头 - VIDEO - 上热门 - 获取
// 2026/8/6 19:03 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::hotlist::get::VideoHotlistGetPort;

////////

/// # [GET ADAPTER] - hotlist get
#[derive(Debug, Default, Clone)]
pub struct VideoHotlistGetAdapter;

#[async_trait]
impl VideoHotlistGetPort for VideoHotlistGetAdapter {
    async fn get_my_collect_ids(&self, uid: i64, limit: i64, offset: i64) -> Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_he_collect_ids(&self, uid: i64, limit: i64, offset: i64) -> Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
