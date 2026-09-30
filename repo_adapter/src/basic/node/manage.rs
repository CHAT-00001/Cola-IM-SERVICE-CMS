// repo_adapter/src/basic/node/manage.rs -- ADAPTER - BASIC - NODE - 节点管理适配器
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::basic::info::core::cdn::NodeInfo;
use port::basic::node::manage::NodeManagePort;

////////

/// # [MANAGE ADAPTER] - 节点管理适配器
/// * `desc`: `COLA BASIC - Node Manage Adapter`
#[derive(Debug, Default, Clone)]
pub struct NodeManageAdapter;

#[async_trait]
impl NodeManagePort for NodeManageAdapter {
    async fn admin_list(
        &self,
        uid: i64,
        user_id: Option<i64>,
        video_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        status_code: i16,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<NodeInfo>, u64)> {
        todo!()
    }
}

//////// END
