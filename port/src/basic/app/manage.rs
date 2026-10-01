// port/src/aaaa/app/manage.rs -- PORT - BASIC - APP - MANAGE
// 2026/8/5 15:23 Created.
 
////////

use cola_data::basic::info::core::app::AppInfo;

////////

/// # [MANAGE PORTS] - 管理
/// * `desc`: `COLA BASIC - APP - MANAGE`
#[async_trait::async_trait]
pub trait AppManagePort: Send + Sync {
    //

    ////////

    /// # [PORT] - 管理员列表
    /// * `desc`: `管理员列表`
    /// * `condition`: `⚠️ WARNING 仅限管理员 / 运营人员`
    async fn admin_list(
        &self,
        app_id: Option<&str>,  // 应用 ID
        keyword: Option<&str>, // 应用搜索关键词
        limit: i64,            // 数量
        offset: i64,           // 偏移
    ) -> anyhow::Result<(Vec<AppInfo>, i64)>;
}

//////// END
