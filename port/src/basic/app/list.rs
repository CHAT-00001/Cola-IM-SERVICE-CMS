// port/src/basic/app/list.rs -- PORT - BASIC - APP  - 列表服务
// 2026/8/5 02:06 Created.

////////

use cola_data::basic::info::core::app::AppInfo;

////////

/// # [LIST SERVICE] - 列表
/// * `desc`: `COLA BASIC - Apps List Service Ports`
#[async_trait::async_trait]
pub trait AppListPort: Send + Sync {
    ////////

    /// # 1. [PORT] - 管理员分页查询存储桶
    /// * `desc`: `查询全部存储桶，支持 app_id 和 keyword 条件`
    async fn admin_find_page(
        &self,
        app_id: Option<&str>,  // 应用 ID
        keyword: Option<&str>, // 搜索关键词
        limit: i64,            // 数量
        offset: i64,           // 偏移
    ) -> anyhow::Result<(Vec<AppInfo>, i64)>;
}

//////// END
