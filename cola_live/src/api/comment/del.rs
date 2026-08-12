// live/api/comment/del.rs
// LIVE - API - COMMENT - DEL
// 2026/8/12 05:56 Created.

////////

use crate::case::comment::del::CommentDelCase;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_auth::info::auth::AuthContext;
use port::app::ctx::AppContext;

////////

/// # [DEL API]
pub struct CommentDelApi;

impl CommentDelApi {
    //

    ////////

    /// # 1. [API HANDLER] - 单条删除
    pub async fn api_delete_comment(
        auth: AuthContext,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<String> {
        // --------

        // 1. 参数校验：comment_id
        let comment_id = match url.comment_id {
            0 => {
                return AppData::err(4002, "缺少comment_id参数", None);
            }
            id => id,
        };

        // 3. Call Case: 删除一条评论
        match CommentDelCase::case_single_delete(auth.uid, comment_id, ctx).await {
            Ok(resp) => AppData::ok(resp), // ✅ 现在的 resp (String) 完美契合 AppData<String>
            Err(e) => {
                tracing::error!("Comment error: {:?}", e);
                AppData::err(6001, "单条评论删除失败", None)
            }
        }
    }
}

//////// END
