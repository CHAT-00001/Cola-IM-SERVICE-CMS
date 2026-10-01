// port/src/aaaa/node/manage.rs -- PORT - BASIC - NODE - 管理服务
// 2026/8/5 15:23 Created.

////////

use cola_data::basic::info::core::cdn::NodeInfo;

////////

/// # [MANAGE PORTS] - 节点管理服务端口
/// * `desc`: `COLA BASIC - Node Manage Service Ports`
#[async_trait::async_trait]
pub trait NodeManagePort: Send + Sync {
    //

    ////////

    /// # [PORT] - 管理员列表
    /// * `desc`: `管理员获取节点列表`
    /// * `condition`: `⚠️ WARNING 仅限管理员 / 运营人员`
    async fn admin_list(
        &self,
        uid: i64,                // 操作者 ID
        region_id: Option<i64>,  // 区域 ID
        node_id: Option<i64>,    // 节点 ID
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        status_code: i16,        // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(Vec<NodeInfo>, u64)>;
}

//////// END
