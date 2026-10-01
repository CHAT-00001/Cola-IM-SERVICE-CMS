// repo_adapter/src/aaaa/node/stat.rs -- ADAPTER - BASIC - NODE - 统计适配器
// 2026/8/6 19:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::basic::node::stat::NodeStatPort;

////////

/// # [STAT ADAPTER] - 节点统计适配器
/// * `desc`: `COLA BASIC - Node Stat Adapter.`
#[derive(Debug, Default, Clone)]
pub struct NodeStatAdapter;

#[async_trait]
impl NodeStatPort for NodeStatAdapter {
    async fn stat_count_by_user_id(&self, uid: i64, user_id: i64) -> Result<(u64)> {
        todo!()
    }

    async fn stat_count_by_video_id(&self, uid: i64, video_id: i64) -> Result<(u64)> {
        todo!()
    }
}

//////// END
