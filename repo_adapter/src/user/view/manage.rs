// repo_adapter/src/user/view/manage.rs
// 🔌 适配器 - 可乐用户 - 浏览 - 管理服务
// 2026/8/7 06:21 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_user::view::manage::UserViewManagePort;

////////

/// # [MANAGE SERVICE] - 管理
/// * `desc`: `用户浏览记录管理服务`
pub struct ViewManageService;

// 构造实现
#[async_trait]
impl UserViewManagePort for ViewManageService {
    //

    ////////

    /// # 1. [ADAPTER] - 管理员列表
    /// * `desc`: `管理员查看所有的浏览记录`
    /// * `condition`: `⚠️ ADMIN / REVIEWER` - `无视身份/权限`
    async fn admin_views_infos(
        &self,
        user_id: i64,
        profile_id: Option<i64>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        status_code: i16,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<i64>)> {
        todo!()
    }
}

//////// END
