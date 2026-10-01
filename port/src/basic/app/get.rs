// port/src/aaaa/app/get.rs -- PORT - BASIC - APP - 获取服务
// 2026/8/14 18:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::basic::info::core::app::AppInfo;

////////

/// # [PORT] - 应用查询服务
/// * `desc`: `COLA BASIC - App Get Service Ports`
#[async_trait]
pub trait AppGetPort: Send + Sync {
    ////////

    /// # 1. [PORT] - 按 app_id 查询
    /// * `desc`: ``
    async fn get_app_by_app_id(
        &self,
        app_id: &str, // 应用 ID
    ) -> Result<Option<AppInfo>>;

    ////////

    /// # 2. [PORT] - 按 ID 查询
    /// * `desc`: ``
    async fn get_app_by_id(
        &self,
        bucket_id: i64, // 存储桶 ID
    ) -> Result<Option<AppInfo>>;
}

//////// END