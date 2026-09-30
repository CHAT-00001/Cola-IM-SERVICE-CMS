// repo_adapter/src/basic/node/del.rs -- ADAPTER - BASIC - NODE - 删除适配器
// 2026/8/6 19:12 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::basic::node::del::NodeDelPort;

////////

/// # [DELETE ADAPTER] - 发布
/// * `desc`: `FS - CDN逻辑删除适配器`
#[derive(Debug, Default, Clone)]
pub struct NodeDelAdapter;

#[async_trait]
impl NodeDelPort for NodeDelAdapter {
    ////////

    /// # 1. [ADAPTER] - 单个删除
    async fn single_delete(&self, node_id: i64) -> Result<(u16)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 批量删除
    async fn batch_delete(&self, node_ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }
}

//////// END
