// port/blacklist.rs  -- 可乐用户中心 - 服务端口层 - 黑名单
// 2026/6/11 20:13

////////

use async_trait::async_trait;
use crate::user::info::config::UserConfigInfo;
use crate::user::info::user::UserInfo;

////////

/// # [SERVICE PORT]
/// * `desc`: 黑名单服务端口
#[async_trait]
pub trait BlacklistPort : Send + Sync + 'static {

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
    async fn get_following(
        &self,
        uid: i64,
    ) -> anyhow::Result<(UserInfo)>;

    ////////

    /// # 4. [PORT] - 删除
    async fn del_following(
        &self,
        uid: i64,
        user_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # 4. [PORT] - 批量删除
    async fn batch_del_following(
        &self,
        uid: i64,
        user_ids: Vec<i64>,
    ) -> anyhow::Result<()>;
}