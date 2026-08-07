// repo_adapter/src/user/view/list.rs
// 🔌 适配器 - 可乐用户 - 浏览 - 列表服务
// 2026/8/6 04:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use cola_data::user::info::config::UserConfigInfo;
use cola_data::user::info::user::UserInfo;
use cola_data::user::port::ban::del::BanDelPort;
use cola_data::user::port::view::list::ViewListPort;
////////

/// # [LIST SERVICE] - 列表
/// * `desc`: `用户浏览记录列表`
pub struct ViewListService;

// 构造实现
#[async_trait]
impl ViewListPort for ViewListService {
    //

    ////////

    /// # 1. [SERVICE] - 单个
    /// * `desc`: `单个软删除`

    async fn get_config(&self, user_id: i64) -> Result<(UserConfigInfo)> {
        todo!()
    }

    async fn add_black(&self, uid: i64, id: i64) -> Result<()> {
        todo!()
    }

    async fn del_black(&self, uid: i64, id: i64) -> Result<()> {
        todo!()
    }

    async fn get_following(&self, uid: i64) -> Result<(UserInfo)> {
        todo!()
    }

    async fn single_del(&self, uid: i64, id: i64) -> Result<(u16)> {
        todo!()
    }

    async fn batch_del(&self, uid: i64, ids: Vec<i64>) -> Result<(u16)> {
        todo!()
    }

    async fn check_state(&self, uid: i64, user_id: i64) -> Result<(bool)> {
        todo!()
    }

    async fn get_list_by_user_id(
        &self,
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_here_list(&self, uid: i64, user_ids: Vec<i64>) -> Result<()> {
        todo!()
    }
}

//////// END
