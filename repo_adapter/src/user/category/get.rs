// repo_adapter/src/user/category/get.rs -- 适配器 - USER - 分类 - 获取适配器
// 🔌 适配器 - 可乐用户 - 分类 - 获取
// 2026/8/10 05:39 Created.

////////

use async_trait::async_trait;
use port::cola_user::category::get::UserCategoryGetPort;

////////

/// # [GET ADAPTER] - 分类适配器
pub struct CategoryGetAdapter;

#[async_trait]
impl UserCategoryGetPort for CategoryGetAdapter {
    async fn get_my_follow_ids(
        &self,
        _uid: i64,
        _id: i64,
        _limit: i64,
        _offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_he_follow_ids(
        &self,
        _uid: i64,
        _id: i64,
        _limit: i64,
        _offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_follow_me_ids(
        &self,
        _uid: i64,
        _id: i64,
        _limit: i64,
        _offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_follow_he_ids(
        &self,
        _uid: i64,
        _id: i64,
        _limit: i64,
        _offset: i64,
    ) -> anyhow::Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
