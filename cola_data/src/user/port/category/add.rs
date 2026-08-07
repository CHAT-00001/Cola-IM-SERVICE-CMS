// user/port/category/add.rs
// 用户 - port - 分类 - 发布
// 2026/8/5 23:34 Created.

////////

use async_trait::async_trait;
use crate::user::command::category::UserCategoryCommand;
use crate::user::info::category::UserCategoryInfo;

////////

/// # [ADD PORTS] - 发布
/// * `desc`: `分类 端口`
#[async_trait]
pub trait CategoryAddPort: Send + Sync + 'static {
    //

    ////////

    /// # [PORT] - 发布
    // 💡 假设 cmd 是你的命令结构体，请根据实际名称替换，例如 UserCategoryCommand
    async fn add_new_one(&self, uid: i64, cmd: UserCategoryCommand) -> anyhow::Result<UserCategoryInfo>;

    ////////

    /// # [PORT] - 编辑
    async fn add_edit_one(&self, uid: i64, cmd: UserCategoryCommand) -> anyhow::Result<UserCategoryInfo>;

    ////////

    /// # [PORT] - 获取
    async fn get_new_list(&self, uid: i64, limit: i64, offset: i64) -> anyhow::Result<Vec<UserCategoryInfo>>;

    ////////

    /// # [PORT] - 批量删除
    async fn batch_del(&self, ids: Vec<i64>) -> anyhow::Result<u16>;
}

//////// END