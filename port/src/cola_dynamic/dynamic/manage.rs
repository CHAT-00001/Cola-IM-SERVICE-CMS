// cola_dynamic/manage.rs
// ⏩️ 端口 - ⏹ 可乐动态 - 动态 - 管理
// 2026/8/5 00:39 Created.

////////

use cola_data::cola_dynamic::info::dynamic::DynamicInfo;

////////

/// # [MANAGE PORTS] - 管理
/// * `desc`: `动态管理端口`
#[async_trait::async_trait]
pub trait ManagePort: Send + Sync {
    //

    ////////

    /// # 1. [PORT] - 全部
    /// * `desc`: `⏹ 可乐动态 - 获取全部动态信息`
    /// * `condition`: `⚠️ ADMIN` - 需要管理员身份
    async fn admin_get_all_dynamic_infos(
        &self,
        user_id: Option<i64>,    // 用户 ID
        keyword: Option<String>, // 关键词
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        status_code: i16,        // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(Vec<DynamicInfo>)>;
}

//////// END
