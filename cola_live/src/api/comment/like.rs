// live/api/identity/like.rs
// LIVE - API - COMMENT - LIKE/DISLIKE
// 2026/8/12 05:57 Created.

////////

use crate::case::comment::like::CommentLikeCase;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::auth::info::auth::AuthContext;
use port::app::ctx::AppContext;
use service::cola_user::user::state::UserStateService;

////////


/// # [LIKE API] - 评论点赞/不喜欢
pub struct CommentLikeApi;

impl CommentLikeApi {
    //

    ////////

    /// # 5. [API HANDLER] - 点赞
    pub async fn api_add_like(
        auth: AuthContext,
        url: ApiGatewayRequest,
        is_liked: bool,
        ctx: &AppContext,
    ) -> AppData<bool> {
        let uid = auth.uid;

        let comment_id = match url.comment_id {
            0 => return AppData::err(4002, "缺少comment_id", None),
            id => id,
        };

        // CALL CASE
        match CommentLikeCase::case_set_comment_like(uid, comment_id, is_liked).await {
            Ok(_) => AppData::ok(true),
            Err(e) => {
                tracing::error!("like identity error: {:?}", e);
                AppData::err(5001, "点赞失败", None)
            }
        }
    }

    ////////

    /// # 6. [API HANDLER] - 不喜欢
    pub async fn api_add_unlike(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<()> {

        let is_disliked = true;
        // CALL CASE
        match CommentLikeCase::case_set_comment_dislike(
            auth.uid,
            url.comment_id,
            is_disliked,
            ctx,
        )
        .await
        {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Unlike Error: {:?}", e);
                AppData::err(5001, "评论不喜欢失败", None)
            }
        }
    }
}

//////// END
