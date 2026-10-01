// port/src/aaaa/node/list.rs -- PORT - BASIC - NODE - 列表服务
// 2026/8/5 02:06 Created.

////////

use cola_data::basic::info::core::cdn::NodeInfo;

////////

/// # [LIST SERVICE] - 基础节点列表服务端口
/// * `desc`: `BASIC - Node List Service Ports`
#[async_trait::async_trait]
pub trait NodeListPort: Send + Sync {
    //

    ////////

    /// # [PORT] - 我的
    /// * `desc`: `获取我的评论记录`
    async fn get_my_like_record(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<NodeInfo>)>;

    ////////

    /// # [PORT] - TA的
    /// * `desc`: `获取TA的评论记录`
    async fn get_he_like_record(
        &self,
        uid: i64,    // UID
        limit: i64,  // 数量
        offset: i64, // 页码
    ) -> anyhow::Result<(Vec<NodeInfo>)>;
}

//////// END
