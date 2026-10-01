// port/src/aaaa/app/add.rs -- PORT - BASIC - APP - 创建服务
// 2026/8/14 18:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::basic::command::core::app::AppCreateCmd;
use cola_data::basic::entity::core::app::AppEntity;

////////

/// # [PORT] - 应用创建服务
/// * `desc`: `COLA BASIC - App Add Service Ports`
#[async_trait]
pub trait AppAddPort: Send + Sync {
    ////////

    /// # 1. [PORT] - 创建应用
    /// * `desc`: `COLA BASIC - 创建一个新应用`
    async fn create_app(&self, cmd: AppCreateCmd) -> Result<AppEntity>;
}

//////// END
