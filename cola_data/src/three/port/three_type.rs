// cola_data/src/three/port/three_type.rs  -- THREE - 类型端口
// 2026/6/18

use crate::three::command::three_type::UpsertTypeCommand;
use crate::three::info::three_type::TypeInfo;

//////

/// # [PORT] - 服务类型端口
#[async_trait::async_trait]
pub trait TypePort: Send + Sync {

    /// 新增/更新
    async fn upsert(&self, cmd: UpsertTypeCommand) -> anyhow::Result<TypeInfo>;

    /// 列表（全部）
    async fn list(&self) -> anyhow::Result<Vec<TypeInfo>>;

    /// 按 code 查询
    async fn find_by_code(&self, code: &str) -> anyhow::Result<Option<TypeInfo>>;
}
