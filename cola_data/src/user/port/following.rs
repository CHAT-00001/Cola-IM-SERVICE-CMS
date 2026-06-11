// port/following.rs  -- 用户中心 - 端口层 - 关注(追随)
// 2026/6/11 20:05

////////

use async_trait::async_trait;
use crate::user::info::config::UserConfigInfo;
use crate::user::info::user::UserInfo;
////////

/// # [SERVICE PORT]
/// * `desc`: 关注服务端口
#[async_trait]
pub trait FollowingPort : Send + Sync + 'static {

    ////////

    /// # 1. [PORT] - 配置
    async fn get_config(
        &self,
        user_id: i64,
    ) -> anyhow::Result<(UserConfigInfo)>;

    ////////

    /// # 2. [PORT] - 添加
    async fn add_following(
        &self,
        uid: i64,
        user_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 获取
    async fn get_following_ids(
        &self,
        uid: i64,
    ) -> anyhow::Result<(Vec<i64>)>;

    /// # 4. [PORT] - 获取
    async fn get_following_infos(
        &self,
        uid: i64,
    ) -> anyhow::Result<(Vec<UserInfo>, Option<i64>)>;

    ////////

    /// # 5. [PORT] - 删除
    async fn del_following(
        &self,
        uid: i64,
        user_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # 6. [PORT] - 批量删除
    async fn batch_del_following(
        &self,
        uid: i64,
        user_ids: Vec<i64>,
    ) -> anyhow::Result<()>;
}