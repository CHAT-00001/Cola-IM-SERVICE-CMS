// cola_user/port/vip/list.rs
// 用户 - port - 贵宾 - 列表
// 2026/8/6 00:35 Created.

////////

use crate::cola_user::info::config::UserConfigInfo;
use crate::cola_user::info::user::UserInfo;
use async_trait::async_trait;

////////

/// # [LIST PORT]
/// * `desc`: `用户贵宾列表端口`
#[async_trait]
pub trait VipListPort: Send + Sync + 'static {
    ////////

    /// # 1. [PORT] - 配置
    async fn get_config(&self, user_id: i64) -> anyhow::Result<(UserConfigInfo)>;

    ////////

    /// # 2. [PORT] - 添加
    async fn add_black(&self, uid: i64, id: i64) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 移除
    async fn del_black(&self, uid: i64, id: i64) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 获取
    async fn get_following(&self, uid: i64) -> anyhow::Result<(UserInfo)>;

    ////////

    /// # 4. [PORT] - 删除
    async fn single_del(
        &self,
        uid: i64, // 操作者ID
        id: i64,  // 目录用户ID
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 5. [PORT] - 批量删除
    /// * `desc`: `管理员批量移除黑名单`
    async fn batch_del(
        &self,
        uid: i64,      // 操作者ID
        ids: Vec<i64>, // 目标用户IDs
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 6. [PORT] - 检查状态
    /// * `Parma`: `uid` / `user_id`
    async fn check_state(
        &self,
        uid: i64,     // 操作者ID
        user_id: i64, // 目标用户ID
    ) -> anyhow::Result<(bool)>;

    ////////

    /// # 7. [PORT] - 获取用户的黑名单
    /// * `return` : `cola_user ids`
    async fn get_list_by_user_id(
        &self,
        user_id: i64, // 目标用户ID
        offset: i64,  // 页数
        limit: i64,   // 数量
    ) -> anyhow::Result<(Vec<i64>)>;

    ////////

    /// # 8. [PORT] - 获取TA的黑名单
    async fn get_here_list(
        &self,
        uid: i64,           // 操作者ID
        user_ids: Vec<i64>, // 返回用户IDs
    ) -> anyhow::Result<()>;
}

//////// END

