// cola_user/src/api/user/home.rs -- USER - API - 用户主页聚合
// 2026-10-03 10:13 Created.

////////

use crate::case::user::home::UserHomeCase;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::auth::info::auth::AuthContext;
use cola_data::cola_user::response::profile::ProfileSingleResponse;
use port::app::ctx::AppContext;

////////

/// # [HOME API] - 用户公开主页接口
/// * `desc`: 返回用户基础资料、统计、主页 Count 和对象关系聚合
pub struct UserHomeApi;

impl UserHomeApi {
    ////////

    /// # 1. [API HANDLER] - 获取用户公开主页
    /// * `desc`: `url.user_id` 有值时查看目标用户，否则查看当前用户
    pub async fn home_get(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<ProfileSingleResponse> {
        let current_uid = auth.uid;
        let target_uid = if url.user_id > 0 {
            url.user_id
        } else {
            current_uid
        };

        match UserHomeCase::case_get_public_home(current_uid, target_uid, ctx).await {
            Ok(response) => {
                tracing::info!(
                    "[🗣️ API] - ✅️ 获取用户主页成功: current_uid={}, target_uid={}",
                    current_uid,
                    target_uid
                );
                AppData::ok(response)
            }
            Err(error) => {
                tracing::error!(
                    "[🤐 API] - ❌️ 获取用户主页失败: current_uid={}, target_uid={}, error={}",
                    current_uid,
                    target_uid,
                    error
                );
                AppData::err(5001, "获取用户主页失败", Some(error.to_string()))
            }
        }
    }
}

//////// END
