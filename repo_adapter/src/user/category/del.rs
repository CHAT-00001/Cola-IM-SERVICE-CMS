// repo_adapter/src/user/category/del.rs -- 适配器 - USER - 分类 - 删除适配器
// 2026/8/10 04:16 Created.

////////

use async_trait::async_trait;
use port::cola_user::category::del::UserCategoryDeletePort;

////////

/// # [DELETE ADAPTER] - 用户分类删除适配器
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
