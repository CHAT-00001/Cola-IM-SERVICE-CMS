// cola_data/src/three/port/three_biz_binding.rs  -- THREE - 绑定端口
// 2026/6/18

use crate::three::command::three_biz_binding::UpsertBindingCommand;
use crate::three::info::three_biz_binding::BindingInfo;

//////

/// # [PORT] - 业务绑定端口
#[async_trait::async_trait]
pub trait BindingPort: Send + Sync {

    /// 新增/更新
    async fn upsert(&self, cmd: UpsertBindingCommand) -> anyhow::Result<BindingInfo>;

    /// 列表（全部）
    async fn list(&self) -> anyhow::Result<Vec<BindingInfo>>;

    /// 按业务模块+类型查询
    async fn find_by_biz(&self, biz_module: &str, biz_type: &str) -> anyhow::Result<Option<BindingInfo>>;
}
