// repo_adapter/src/aaaa/app/manage.rs -- ADAPTER - BASIC - 应用 -管理
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::basic::info::core::app::AppInfo;
use port::basic::app::manage::AppManagePort;
use repository::basic::pg::app::AppRepo;
use repository::pg_pool;

////////

/// # [ADD ADAPTER] - 管理
/// * `desc`: `AUTH - 验证身份管理适配器`
#[derive(Debug, Default, Clone)]
pub struct AppManageAdapter;

#[async_trait]
impl AppManagePort for AppManageAdapter {
    async fn admin_list(
        &self,
        app_id: Option<&str>,
        keyword: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<AppInfo>, i64)> {
        let (entities, total) =
            AppRepo::admin_find_page(&pg_pool(), app_id, keyword, limit, offset).await?;
        let infos = entities.into_iter().map(Into::into).collect();
        Ok((infos, total))
    }
}

//////// END
