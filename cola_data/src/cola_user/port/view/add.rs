// cola_user/port/view/add.rs
// 用户 - port - 浏览 - 发布
// 2026/8/6 00:44 Created.

////////

////////

use crate::cola_user::command::new::UserCommand;

/// # [ADD PORTS] - 浏览
#[async_trait::async_trait]
pub trait ViewAddPort: Send + Sync + 'static {
    ////////

    /// # [SERVICE] - 创建
    async fn save_view(&self, cmd: UserCommand) -> anyhow::Result<()>;

    ////////

    /// # [SERVICE] - 编辑
    async fn del_view(&self, cmd: UserCommand) -> anyhow::Result<()>;

    ////////

    /// # [SERVICE] - 删除一个
    async fn del_one_user(&self, user_id: i64) -> anyhow::Result<()>;

    ////////

    /// # [SERVICE] - 删除多个
    async fn del_many_user(&self, user_ids: Vec<i64>) -> anyhow::Result<()>;
}

//////// END