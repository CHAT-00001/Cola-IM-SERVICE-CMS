// /view.rs  -- 
// 2026/6/10 07:36

////////

use crate::user::command::user::UserCommand;

////////

/// # [SERVICE] - 浏览
#[async_trait::async_trait]
pub trait ViewPort : Send + Sync + 'static {


    ////////

    /// # [SERVICE] - 创建
    async fn save_view(
        &self,
        cmd: UserCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # [SERVICE] - 编辑
    async fn del_view(
        &self,
        cmd: UserCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # [SERVICE] - 删除一个
    async fn del_one_user(
        &self,
        user_id: i64,
    ) -> anyhow::Result<()>;

    ////////

    /// # [SERVICE] - 删除多个
    async fn del_many_user(
        &self,
        user_ids: Vec<i64>,
    ) -> anyhow::Result<()>;
}