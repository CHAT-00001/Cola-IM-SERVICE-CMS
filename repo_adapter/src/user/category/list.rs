// repo_adapter/src/user/category/list.rs
// 🔌 适配器 - 可乐用户 - 分类 - 列表
// 2026/8/10 04:16 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::category::UserCategoryInfo;
use port::cola_user::category::list::UserCategoryListPort;

////////

/// # [LIST ADAPTER] - 分类列表适配器
pub struct CategoryListAdapter;

#[async_trait]
impl UserCategoryListPort for CategoryListAdapter {
    async fn get_new_list(&self, _limit: i64, _offset: i64) -> anyhow::Result<(Vec<UserCategoryInfo>)> {
        todo!()
    }

    async fn get_hot_list(&self, _limit: i64, _offset: i64) -> anyhow::Result<(Vec<UserCategoryInfo>)> {
        todo!()
    }
}

//////// END
