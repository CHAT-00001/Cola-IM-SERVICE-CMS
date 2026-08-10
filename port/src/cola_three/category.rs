// cola_data/src/cola_three/category.rs
// ⏩️ 端口 - 可乐三方 - 分类
// 2026/7/27 14:14 Created.

////////

use cola_data::cola_three::command::category::ThreeServerTypeCommand;
use cola_data::cola_three::info::server_type::ServerTypeInfo;

////////

/// # [PORT] - 服务分类端口
/// * `desc`: `用于创建/管理第三方服务的分类（有哪些服务分类）`
#[async_trait::async_trait]
pub trait TypePort: Send + Sync {

    /// 新增/更新服务分类
    async fn upsert(&self, cmd: ThreeServerTypeCommand) -> anyhow::Result<ServerTypeInfo>;

    /// 列表（全部）
    async fn list(&self) -> anyhow::Result<Vec<ServerTypeInfo>>;

    /// 按 code 查询
    async fn find_by_code(&self, code: &str) -> anyhow::Result<Option<ServerTypeInfo>>;
}

//////// END