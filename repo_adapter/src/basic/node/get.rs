// repo_adapter/src/aaaa/node/get.rs -- ADAPTER - BASIC - NODE - 获取适配器
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::aaaa::info::core::cdn::NodeInfo;
use port::basic::node::get::NodeGetPort;

////////

/// # [GET ADAPTER] - 节点获取适配器
/// * `desc`: `COLA BASIC - Node Get Service Adapter`
#[derive(Debug, Default, Clone)]
pub struct NodeGetAdapter;

#[async_trait]
impl NodeGetPort for NodeGetAdapter {
    ////////

    /// # 1. [ADAPTER] - 单个节点
    async fn get_node_info_by_id(&self, node_id: i64) -> Result<(NodeInfo)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 附近的节点
    async fn get_node_infos_by_geo(
        &self,
        lat: f64,    // 纬度
        lng: f64,    // 经度
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> Result<(Vec<NodeInfo>)> {
        todo!()
    }

    ////////

    /// # 3. [ADAPTER] - 区域的
    async fn get_node_infos_by_region_id(
        &self,
        region_id: i64, // 区域 ID
        limit: i64,     // 数量
        offset: i64,    // 页码
    ) -> Result<(Vec<NodeInfo>)> {
        todo!()
    }
}

//////// END
