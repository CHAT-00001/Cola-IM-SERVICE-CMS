// /add.rs  -- 
// 2026/6/10 07:31

////////

use crate::user::command::user::UserCommand;


#[async_trait::async_trait]
pub trait AddPort : Send + Sync + 'static {


    ////////

    /// # [SERVICE] - 创建
    async fn save_user(
        &self,
        cmd: UserCommand,
    ) -> anyhow::Result<()>;

    ////////

    /// # [SERVICE] - 编辑
    async fn edit_user(
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