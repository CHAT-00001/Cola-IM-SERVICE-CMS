// repo_adapter/src/cola_user/view/list.rs
// 🔌 适配器 - 可乐用户 - 浏览 - 列表服务
// 2026/8/6 04:18 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::view::list::UserViewListPort;

////////

/// # [LIST ADAPTER] - 列表
/// * `desc`: `USER - 用户主页浏览记录列表`
pub struct ViewListService;

// 构造实现
#[async_trait]
impl UserViewListPort for ViewListService {
    //

    ////////

    /// # 1. [ADAPTER] - 用户主动看的
    async fn get_view_infos_by_user_id(
        &self,
        user_id: i64, // 用户 ID
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<i64>)> {
        todo!()
    }

    ////////

    /// # 2. [ADAPTER] - 资料被动看的
    async fn get_view_infos_by_profile_id(
        &self,
        profile_id: i64, // 资料 ID
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
