// repo_adapter/src/user/category/add.rs
// 🔌 适配器 - 可乐用户 - 分类 - 发布
// 2026/8/6 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::cola_user::command::category::UserCategoryCommand;
use cola_data::cola_user::info::category::UserCategoryInfo;
use port::cola_user::category::add::UserCategoryAddPort;

////////

/// # [ADD ADAPTER] - 发布
/// * `desc`: `🗣 用户 - 分类发布`
pub struct CategoryAddAdapter;

#[async_trait]
impl UserCategoryAddPort for CategoryAddAdapter {
    //

    ////////

    /// # [ADAPTER] - 发布
    async fn add_new_one(&self, _uid: i64, _cmd: UserCategoryCommand) -> Result<UserCategoryInfo> {
        todo!()
    }

    ////////

    /// # [ADAPTER] - 编辑
    async fn add_edit_one(&self, _uid: i64, _cmd: UserCategoryCommand) -> Result<UserCategoryInfo> {
        todo!()
    }

    ////////

    /// # [ADAPTER] - 获取
    async fn get_new_list(
        &self,
        _uid: i64,
        _limit: i64,
        _offset: i64,
    ) -> Result<Vec<UserCategoryInfo>> {
        todo!()
    }

    ////////

    /// # [ADAPTER] - 批量删除
    async fn batch_del(&self, _ids: Vec<i64>) -> Result<u16> {
        todo!()
    }
}

//////// END
