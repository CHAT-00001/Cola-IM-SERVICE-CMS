// repo_adapter/src/basic/node/add.rs -- ADAPTER - BASIC - NODE - 发布服务
// 2026/8/8 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::basic::command::core::node::{NodeCreateCmd, NodeUpdateCmd};
use cola_data::basic::info::core::cdn::NodeInfo;
use port::basic::node::add::NodeAddPort;

////////

/// # [ADD ADAPTER] - 节点发布适配器
/// * `desc`: `BASIC - Node Add Adapter.`
#[derive(Debug, Default, Clone)]
pub struct NodeAddAdapter;

#[async_trait]
impl NodeAddPort for NodeAddAdapter {
    async fn create_node(&self, uid: i64, cmd: NodeCreateCmd) -> Result<(NodeInfo)> {
        todo!()
    }

    async fn update_node(&self, uid: i64, node_id: i64, cmd: NodeUpdateCmd) -> Result<(NodeInfo)> {
        todo!()
    }
}

//////// END
