// repo_adapter/src/user/view/add.rs -- 适配器 - USER - 浏览记录 - 获取适配器
// 2026/8/6 04:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::view::get::UserViewGetPort;

////////

/// # [GET ADAPTER] - 用户浏览获取服务
/// * `desc`: `COLA USER - View Record Get Adapter`
pub struct UserViewGetAdapter;

// 构造实现
#[async_trait]
impl UserViewGetPort for UserViewGetAdapter {
    async fn get_views_ids(&self, user_id: i64, limit: i64, offset: i64) -> Result<(Vec<i64>)> {
        todo!()
    }

    async fn get_view_me_ids(&self, user_id: i64, limit: i64, offset: i64) -> Result<(Vec<i64>)> {
        todo!()
    }
    //
}

//////// END
