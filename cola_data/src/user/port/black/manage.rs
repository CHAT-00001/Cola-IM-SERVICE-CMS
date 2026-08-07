// user/port/black/manage.rs
// 用户 - port - 黑名单 - 管理
// 2026/8/5 21:35 Created.

////////

use crate::user::info::black::UserBlackInfo;
use async_trait::async_trait;

////////

/// # [MANAGE PORTS]
/// * `desc`: `用户黑名单管理端口`
/// * `warning`: `⚠️ 黑名单是私密行为, 不需要管理接口`
#[async_trait]
pub trait BlackManagePort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 列表
    /// * `desc`: `获取黑名单记录`
    async fn get_black_list(
        &self,
        user_id: i64, // 目标用户ID
        offset: i64,  // 页数
        limit: i64,   // 数量
    ) -> anyhow::Result<(Vec<UserBlackInfo>)>;
}

//////// END
