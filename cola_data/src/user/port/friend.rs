// port/friend.rs  -- 用户中心 - 端口层 - 朋友(追随)
// 2026/6/11 20:14

use async_trait::async_trait;
use crate::user::info::config::UserConfigInfo;
use crate::user::info::user::UserInfo;

////////

/// # [SERVICE PORT]
/// * `desc`: 朋友服务端口
#[async_trait]
pub trait FriendPort : Send + Sync + 'static {

    ////////

    /// # 1. [PORT] - 配置
    async fn get_config(
        &self,
        user_id: i64,
    ) -> anyhow::Result<(UserConfigInfo)>;

    ////////

    /// # 2. [PORT] - 添加
    async fn add_friend(
        &self,
        uid: i64,
        user_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 获取
    async fn get_friend(
        &self,
        uid: i64,
    ) -> anyhow::Result<(UserInfo)>;

    ////////

    /// # 4. [PORT] - 删除
    async fn del_friend(
        &self,
        uid: i64,
        user_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - 批量删除
    async fn batch_del_friend(
        &self,
        uid: i64,
        user_ids: Vec<i64>,
    ) -> anyhow::Result<()>;
}