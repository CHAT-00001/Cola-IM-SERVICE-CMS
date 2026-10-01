// repo_adapter/src/aaaa/app/list.rs -- ADAPTER - BASIC - 应用 - 列表服务
// 2026/8/6 18:55 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::basic::info::core::app::AppInfo;
use port::basic::app::list::AppListPort;
use repository::basic::pg::app::AppRepo;
use repository::pg_pool;

////////

/// # [LIAT ADAPTER] - 列表
/// * `desc`: `FS - 存储桶列表适配器`
#[derive(Debug, Default, Clone)]
pub struct AppListAdapter;

#[async_trait]
impl AppListPort for AppListAdapter {
    ////////

    /// # [ADAPTER] - 管理员列表
    /// * `desc`: `管理员获取应用列表`
    async fn admin_find_page(
        &self,
        app_id: Option<&str>,
        keyword: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<AppInfo>, i64)> {
        ////////

        // 💡 - ENTITIES - 应用实体
        let (entities, total) =
            AppRepo::admin_find_page(&pg_pool(), app_id, keyword, limit, offset).await?;

        ////////

        // 💡 - INFOS - 转换成信息
        let list: Vec<AppInfo> = entities.into_iter().map(Into::into).collect();

        ////////

        // 💡 - LOGGER
        tracing::info!(
            "[🔌 ADAPTER] - ✅️ 管理员应用列表查询成功: app_id={:?}, count={}, total={}",
            app_id,
            list.len(),
            total
        );
        Ok((list, total))
    }
}

//////// END
