// port/src/basic/node/del.rs -- 端口 - BASIC - 节点 - 删除服务
// 2026/8/5 00:03 Created.

////////

////////

/// # [DELETE SERVICE] - 基础节点删除服务
/// `desc`: `BASIC - Node Delete Service Port`
#[async_trait::async_trait]
pub trait NodeDelPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 单个删除
    async fn single_delete(
        &self,
        node_id: i64, // 节点 ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 2. [PORT] - 批量删除
    async fn batch_delete(
        &self,
        node_ids: Vec<i64>, // 节点 IDs
    ) -> anyhow::Result<(u16)>;
}

//////// END
