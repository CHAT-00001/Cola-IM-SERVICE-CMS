// port\src\basic\node\config.rs -- PORT - BASIC - NODE - 配置管理
// 2026/8/16 Created.

////////

use cola_data::cola_fs::command::cdn::{CreateCdnDomainCmd, UpdateCdnDomainCmd};
use cola_data::cola_fs::info::cdn::CdnDomainInfo;

////////

/// # 1. [PORT] - 节点配置管理
/// * `desc`: `COLA BASIC - Node Config Ports`
#[async_trait::async_trait]
pub trait NodeConfigPort: Send + Sync {
    /// # 1. [PORT] - 创建 CDN 配置
    async fn create(&self, cmd: CreateCdnDomainCmd) -> anyhow::Result<CdnDomainInfo>;

    ////////

    /// # 2. [PORT] - 更新 CDN 配置
    async fn update(&self, id: i64, cmd: UpdateCdnDomainCmd) -> anyhow::Result<CdnDomainInfo>;

    ////////

    /// # 3. [PORT] - 更新 CDN 状态
    async fn update_status(&self, id: i64, status: i16) -> anyhow::Result<CdnDomainInfo>;

    ////////

    /// # 4. [PORT] - 按应用查询启用配置
    async fn find_by_app_id(&self, app_id: &str) -> anyhow::Result<Option<CdnDomainInfo>>;

    ////////

    /// # 5. [PORT] - 按应用与桶键查询启用配置
    async fn find_by_bucket_key(
        &self,
        app_id: Option<&str>,
        bucket_key: &str,
    ) -> anyhow::Result<Option<CdnDomainInfo>>;

    ////////

    /// # 6. [PORT] - 按 ID 查询配置
    async fn find_by_id(&self, id: i64) -> anyhow::Result<Option<CdnDomainInfo>>;

    ////////

    /// # 7. [PORT] - 分页查询 CDN 配置列表
    async fn list(
        &self,
        app_id: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<CdnDomainInfo>, i64)>;

    ////////

    /// # 8. [PORT] - 逻辑删除配置
    async fn delete(&self, id: i64) -> anyhow::Result<u64>;
}

//////// END
