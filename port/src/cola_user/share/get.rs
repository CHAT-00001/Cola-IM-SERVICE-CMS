// port/src/user/share/get.rs -- 端口 - USER - 分享 - 获取端口
// 2026/6/11 20:13

////////

use async_trait::async_trait;

////////

/// # [GET PORTS]
/// * `desc`: `COLA USER - Share Get Ports`
#[async_trait]
pub trait UserShareGetPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 用户分享的
    /// * `desc`: `根据用户ID` - `获取用户分享的主页IDs`
    async fn get_share_ids(
        &self,
        user_id: i64, // 用户ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 2. [PORT] - 被分享的用户
    /// * `desc`: `根据用户ID` - `获取用户主页被谁分享了`
    async fn get_share_me_ids(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,   // 数量
        offset: i64,  // 页码
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
