// repo_adapter/src/aaaa/app/add.rs -- ADAPTER - BASIC - APP - 发布服务
// 2026/8/14 18:00 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::aaaa::command::core::app::AppCreateCmd;
use cola_data::aaaa::entity::core::app::AppEntity;
use port::basic::app::add::AppAddPort;
use repository::basic::pg::app::AppRepo;
use repository::pg_pool;

////////

/// # [ADD ADAPTER] - 应用创建适配器
/// * `desc`: `COLA BASIC - App Add Adapter`
#[derive(Debug, Default, Clone)]
pub struct AppAddAdapter;

#[async_trait]
impl AppAddPort for AppAddAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 创建应用
    async fn create_app(&self, cmd: AppCreateCmd) -> Result<AppEntity> {
        let app_id = cmd.app_id.clone().unwrap_or_default();
        if !app_id.trim().is_empty() && AppRepo::exists_by_app_id(&pg_pool(), &app_id, None).await?
        {
            return Err(anyhow::anyhow!("应用 app_id 已存在: {}", app_id));
        }

        let bucket = AppRepo::create(&pg_pool(), cmd).await?;

        tracing::info!("[🔌 ADAPTER] - ✅️ 应用创建成功: app_id={}", app_id,);

        Ok(bucket)
    }
}

//////// END
