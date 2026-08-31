// port/src/cola_three/app.rs
// ⏩️ 端口 - 可乐三方 - 应用管理端口
// 2026/8/15 13:10 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_three::info::app::AppInfo;

////////

/// # [PORT] - 应用管理端口
#[async_trait]
pub trait AppPort: Send + Sync {
    ////////
    /// # 1. [PORT] - 按 app_id 获取应用信息
    async fn get_app_by_app_id(&self, app_id: &str) -> Result<Option<AppInfo>>;

    ////////
    /// # 2. [PORT] - 获取应用列表
    async fn list_apps(&self) -> Result<Vec<AppInfo>>;

    ////////
    /// # 3. [PORT] - 创建应用
    async fn create_app(
        &self,
        app_id: &str,
        name: &str,
        description: Option<String>,
        status: i16,
    ) -> Result<AppInfo>;
}

//////// END
