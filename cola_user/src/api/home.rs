// cola_user/src/api/home.rs  -- 用户中心 - 接口层 - 主页
// 2026/6/18 09:06

//////

use crate::case::home::UserHomeCase;
use crate::model::vo::user::UserVo;
use cola_data::app::ctx::AppContext;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;

//////

/// # [HOME API] - 用户主页 接口
pub struct HomeApi;

impl HomeApi {

    ////////

    /// # 1. [API HANDLER] - 获取最新注册用户
    /// 对应 gateway service = "home.new"
    pub async fn handler_get_new(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<Vec<UserVo>> {

        let uid = auth.uid;

        match UserHomeCase::case_get_newest_users(uid, url, ctx).await {
            Ok(users) => AppData::ok(users),

            Err(e) => {
                tracing::error!("获取最新用户列表失败: {:?}", e);
                AppData::err(5001, "获取最新用户列表失败", None)
            }
        }
    }
}
