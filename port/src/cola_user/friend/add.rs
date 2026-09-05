// user/port/friend/add.rs -- 端口 - USER - 朋友 - 发布端口
// 2026/8/5 21:58 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::user::UserInfo;

////////

/// # [ADD PORTS] - 用户朋友发布端口
/// * `desc`: `COLA USER - Friend Add Ports`
#[async_trait]
pub trait FriendAddPort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 创建朋友
    async fn create_friend(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<()>;

    ////////

    /// # 2. [PORT] - 移除朋友
    async fn delete_friend(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
    ) -> anyhow::Result<()>;

    ////////

    /// # 3. [PORT] - 更新/插入朋友
    async fn upsert_friend(
        &self,
        uid: i64,     // UID
        user_id: i64, // 用户 ID
        status: i16,  // 状态码
    ) -> anyhow::Result<()>;

    /// # 3. [PORT] - 获取
    async fn get_friending(&self, uid: i64) -> anyhow::Result<(UserInfo)>;

    ////////

    /// # 4. [PORT] - 更新朋友备注
    async fn update_friend(
        &self,
        uid: i64,               // 操作者ID
        id: i64,                // 目录用户ID
        remark: Option<String>, // 备注
    ) -> anyhow::Result<(u16)>;

    ////////

    /// # 5. [PORT] - 批量删除
    /// * `desc`: `管理员批量移除朋友`
    async fn batch_delete(
        &self,
        uid: i64,      // 操作者ID
        ids: Vec<i64>, // 目标用户IDs
    ) -> anyhow::Result<(u16)>;
}

//////// END
