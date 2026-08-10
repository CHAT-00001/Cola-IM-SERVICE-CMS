// port/src/cola_user/black/list.rs
// ⏩️ 端口 - 🗣 可乐用户 - 黑名单 - 审计日志列表
// 2026/8/5 21:36 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::black::UserBlackInfo;

////////

/// # [LIST PORT] - 列表
/// * `desc`: `黑名单审计日志列表端口`
#[async_trait]
pub trait UserBlackListPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 多条件组合筛选获取黑名单审计日志
    /// * `desc`: `全能接口：支持按操作者、目标用户、起止时间分页查询黑名单操作日志`
    async fn get_black_list(
        &self,
        actor_id: Option<i64>,   // 谁
        target_id: Option<i64>,  // 拉黑了谁
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 截止时间
        limit: i64,              // 数量
        offset: i64,             // 页码
    ) -> anyhow::Result<(i64, Vec<UserBlackInfo>)>; // 返回 (总条数 total, 日志列表)
}

//////// END
