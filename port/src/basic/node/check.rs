// port/src/aaaa/node/check.rs -- PORT - BASIC - 节点 - 检查服务
// 2026/8/5 00:00 Created.

////////

////////

/// # [CHECK PORTS] - 节点检查服务端口
/// * `desc`: `COLA BASIC - Node Check Service Ports`
#[async_trait::async_trait]
pub trait NodeCheckPort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 健康
    /// * `desc`: `检查目标健康`
    async fn check_health(
        &self,
        uid: i64,     // UID
        node_id: i64, // 节点 ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 2. [PORT] - 状态
    /// * `desc`: `检查目标状态`
    async fn check_state(
        &self,
        uid: i64,     // UID
        node_id: i64, // 节点 ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 3. [PORT] - 是否所有者
    /// * `desc`: `根据用户ID + 评论ID` - `检查是否属于作者`
    async fn is_owner(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        node_id: i64, // 节点 ID
    ) -> anyhow::Result<(bool)>;
}

//////// END
