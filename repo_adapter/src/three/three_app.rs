// repo_adapter/src/three/three_app.rs -- 适配器 - THREE - 应用管理
// 2026/8/15 13:10 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_three::info::app::AppInfo;
use port::cola_three::app::AppPort;
use repository::cola_three::pg::app_repo::AppRepo;

////////

/// # [ADAPTER] - 应用管理适配器
#[derive(Debug, Default, Clone)]
pub struct AppAdapter;

#[async_trait]
impl AppPort for AppAdapter {
    ////////
    async fn get_app_by_app_id(&self, app_id: &str) -> Result<Option<AppInfo>> {
        let entity_opt = AppRepo::find_by_app_id(app_id).await?;
        match entity_opt {
            Some(e) => Ok(Some(e.to_app_info()?)),
            None => Ok(None),
        }
    }

    ////////
    async fn list_apps(&self) -> Result<Vec<AppInfo>> {
        let entities = AppRepo::list().await?;
        let mut infos = Vec::with_capacity(entities.len());
        for e in entities {
            infos.push(e.to_app_info()?);
        }
        Ok(infos)
    }

    ////////
    async fn create_app(
        &self,
        app_id: &str,
        name: &str,
        description: Option<String>,
        status: i16,
    ) -> Result<AppInfo> {
        let entity = AppRepo::insert(app_id, name, description.as_deref(), status).await?;
        Ok(entity.to_app_info()?)
    }
}

//////// END
