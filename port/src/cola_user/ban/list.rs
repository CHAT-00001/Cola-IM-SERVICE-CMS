// port/src/cola_user/ban/list.rs
// ⏩️ 端口 - 🗣 可乐用户 - 封禁 - 列表
// 2026/8/5 21:36 Created.

////////

use async_trait::async_trait;

////////

/// # [LIST PORT]
/// * `desc`: `用户封禁列表端口`
#[async_trait]
pub trait UserBanListPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 获取封禁信息
    /// * `desc` : `获取封禁记录信息,用户审计审查`
    async fn get_ban_list(
        &self,
        user_id: i64, // 目标用户ID
        offset: i64,  // 页数
        limit: i64,   // 数量
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
