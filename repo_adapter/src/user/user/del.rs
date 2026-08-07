// repo_adapter/src/cola_user/cola_user/del.rs
// 🔌 适配器 - 用户 - 用户 - 列表服务
// 2026/8/7 04:51 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::port::user::del::UserDelPort;

////////

/// # [DEL SERVICE] - 删除
/// * `desc`: `用户删除服务`
pub struct UserDelAdapter;

// 构造实现
#[async_trait]
impl UserDelPort for UserDelAdapter {
    //

    ////////

    /// # 1. [SERVICE] - 单个软删除
    /// * `desc`: `用户注销时`
    async fn single_soft_del(
        &self,
        uid: i64, // UID
        id: i64,  // 目标ID
    ) -> anyhow::Result<(u16)> {
        todo!()
    }

    ////////

    /// # 2. [SERVICE] - 批量软删除
    /// * `desc`: `管理员批量软删除用户`
    async fn batch_soft_del(
        &self,
        uid: i64,      // UID
        ids: Vec<i64>, // 目标IDs
    ) -> anyhow::Result<(u16)> {
        todo!()
    }
}

//////// END
