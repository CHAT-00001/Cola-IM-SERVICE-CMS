// repo_adapter/src/basic/node/list.rs -- ADAPTER - BASIC - NODE - 列表适配器
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::basic::info::core::cdn::NodeInfo;
use port::basic::node::list::NodeListPort;

////////

/// # [LIAT ADAPTER] - 列表
/// * `desc`: `AUTH - 验证身份列表适配器`
#[derive(Debug, Default, Clone)]
pub struct NodeListAdapter;

#[async_trait]
impl NodeListPort for NodeListAdapter {
    async fn get_my_like_record(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<NodeInfo>)> {
        todo!()
    }

    async fn get_he_like_record(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<NodeInfo>)> {
        todo!()
    }
}

//////// END
