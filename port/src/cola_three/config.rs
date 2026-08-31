// port/src/cola_three/port/config.rs
// ⏩️ 端口 - 可乐三方 - 配置端口
// 2026/6/18

////////

use cola_data::cola_three::command::config::UpsertConfigCommand;
use cola_data::cola_three::info::config::ConfigInfo;

////////

/// # [PORT] - 服务配置端口
#[async_trait::async_trait]
pub trait ConfigPort: Send + Sync {
    /// 新增/更新
    async fn upsert(&self, cmd: UpsertConfigCommand) -> anyhow::Result<ConfigInfo>;

    /// 按 type_id 查询列表
    async fn list_by_type(&self, type_id: i64) -> anyhow::Result<Vec<ConfigInfo>>;

    /// 按 ID 查询
    async fn find_by_id(&self, id: i64) -> anyhow::Result<Option<ConfigInfo>>;

    /// 按 biz_module + biz_type 查绑定配置（JOIN 查询）
    async fn find_binded(
        &self,
        biz_module: &str,
        biz_type: &str,
    ) -> anyhow::Result<Option<ConfigInfo>>;
}
