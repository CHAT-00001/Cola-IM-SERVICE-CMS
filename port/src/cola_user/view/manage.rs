// port/src/user/view/manage.rs -- 端口 - USER - 浏览 - 管理端口
// 2026/8/6 00:44 Created.

////////

use async_trait::async_trait;
use cola_data::cola_user::info::config::UserConfigInfo;
use cola_data::cola_user::info::user::UserInfo;
////////

/// # [MANAGE PORTS]
/// * `desc`: `用户浏览管理端口`
#[async_trait]
pub trait UserViewManagePort: Send + Sync + 'static {
    //

    ////////

    /// # 1. [PORT] - 管理员列表
    /// * `desc` : `USER - 管理员查看浏览记录信息`
    async fn get_admin_list(
        &self,
        user_id: i64,            // 用户 ID
        profile_id: Option<i64>, // 主页 ID
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        status_code: i16,        // 状态码
        limit: i64,              // 数量
        offset: i64,             // 页数
    ) -> anyhow::Result<(Vec<i64>)>;
}

//////// END
