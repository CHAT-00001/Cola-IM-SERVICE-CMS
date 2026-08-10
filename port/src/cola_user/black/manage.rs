// port/src/cola_user/black/manage.rs
// ⏩️ 端口 - 🗣 可乐用户 - 黑名单 - 管理
// 2026/8/5 21:35 Created.

////////


use async_trait::async_trait;
use cola_data::cola_user::info::black::UserBlackInfo;

////////

/// # [MANAGE PORTS]
/// * `desc`: `用户黑名单管理端口`
/// * `warning`: `⚠️ 黑名单是私密行为, 不需要管理接口`
#[async_trait]
pub trait UserBlackManagePort: Send + Sync + 'static {
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
