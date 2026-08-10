// repo_adapter/src/cola_video/recommend/get.rs
// 🔌 插头 - 可乐视频 - 推荐 - 获取
// 2026/8/6 18:59 Created.

////////

use async_trait::async_trait;
use port::cola_video::recommend::get::VideoRecommendGetPort;

////////

/// # [GET ADAPTER] - recommend get
#[derive(Debug, Default, Clone)]
pub struct VideoRecommendGetAdapter;

// 构造实现
#[async_trait]
impl VideoRecommendGetPort for VideoRecommendGetAdapter {
    async fn get_my_collect_ids(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_he_collect_ids(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
