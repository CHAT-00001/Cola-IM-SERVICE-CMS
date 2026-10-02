// port/src/aaaa/node/get.rs -- PORT - BASIC - NODE - 获取服务
// 2026/6/10 08:23 Created.

////////

use cola_data::aaaa::info::core::cdn::NodeInfo;

////////

/// # [GET SERVICE PORT] - 基础节点获取服务端口
/// * `desc`: `BASIC - Node Get Service Ports`
#[async_trait::async_trait]
pub trait NodeGetPort: Send + Sync {
    ////////

    /// # [PORT] - 单个
    async fn get_node_info_by_id(
        &self,
        node_id: i64, // 节点 ID
    ) -> anyhow::Result<(NodeInfo)>;

    ////////

    /// # [PORT] - 附近
    async fn get_node_infos_by_geo(
        &self,
        lat: f64,    // 纬度
        lng: f64,    // 经度
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<NodeInfo>)>;

    ////////

    /// # [PORT] - 区域的
    async fn get_node_infos_by_region_id(
        &self,
        region_id: i64, // 节点 ID
        limit: i64,     // 数量
        offset: i64,    // 页码
    ) -> anyhow::Result<(Vec<NodeInfo>)>;
}

//////// END
