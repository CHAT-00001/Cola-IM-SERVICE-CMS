// repo_adapter/src/cola_user/cola_user/list.rs
// 🔌 适配器 - 用户 - 用户 - 列表服务
// 2026/8/6 04:19 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::user::UserInfo;
use port::cola_user::user::list::UserListPort;

////////

/// # [LIST ADAPTER] - 列表
/// * `desc`: `USER - 用户信息列表适配器`
pub struct UserListAdapter;

// 构造实现
#[async_trait]
impl UserListPort for UserListAdapter {
    //

    ////////

    /// # 1. [ADAPTER] - 最新
    /// * `desc`: `保存新用户记录`
    async fn get_new_list(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<UserInfo>)> {
        todo!()
    }

    async fn get_hot_list(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<UserInfo>)> {
        todo!()
    }

    async fn get_recommend_list(
        &self,
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<UserInfo>)> {
        todo!()
    }

    async fn get_city_list(
        &self,
        uid: i64,
        city_id: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<UserInfo>)> {
        todo!()
    }

    async fn get_nearby_list(
        &self,
        uid: i64,
        lat: f64,
        lng: f64,
        range: u32,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<UserInfo>)> {
        todo!()
    }

    async fn get_category_list(
        &self,
        uid: i64,
        category_id: i64,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<UserInfo>)> {
        todo!()
    }

    async fn get_search_list(
        &self,
        uid: i64,
        keyword: String,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<UserInfo>)> {
        todo!()
    }
}

//////// END
