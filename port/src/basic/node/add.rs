// port/src/aaaa/node/add.rs -- PORT  - BASIC - 节点 - 发布服务
// 2026/8/5 00:03 Created.

////////

use cola_data::aaaa::command::core::node::{NodeCreateCmd, NodeUpdateCmd};
use cola_data::aaaa::info::core::cdn::NodeInfo;

////////

/// # [ADD SERVICE] - 基础节点发布服务
/// * `desc`: `BASIC - Node Add Service Ports`
#[async_trait::async_trait]
pub trait NodeAddPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 添加节点
    /// * `desc`: ``
    async fn create_node(
        &self,
        uid: i64,           // UID
        cmd: NodeCreateCmd, // 命令
    ) -> anyhow::Result<(NodeInfo)>;

    ////////

    /// # 2. [PORT] - 编辑节点
    /// * `desc`: ``
    async fn update_node(
        &self,
        uid: i64,           // UID
        node_id: i64,       // 节点 ID
        cmd: NodeUpdateCmd, // 命令
    ) -> anyhow::Result<(NodeInfo)>;
}

//////// END
