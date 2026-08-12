// repo_adapter/src/user/category/del.rs
// 🔌 适配器 - 可乐用户 - 分类 - 删除
// 2026/8/10 04:16 Created.

////////

use async_trait::async_trait;
use port::cola_user::category::del::UserCategoryDeletePort;

////////

/// # [DELETE ADAPTER] - 删除
pub struct CategoryDeleteAdapter;

#[async_trait]
impl UserCategoryDeletePort for CategoryDeleteAdapter {
    async fn single_delete(&self, _id: i64) -> anyhow::Result<(u16)> {
        todo!()
    }

    async fn batch_delete(&self, _ids: Vec<i64>) -> anyhow::Result<(u16)> {
        todo!()
    }
}

//////// END
