// live/api/file/get.rs
// LIVE - API - COMMENT - GET
// 2026/8/12 05:55 Created.

////////

use crate::case::comment::get::CommentGetCase;
use cola_data::app::data::AppData;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::auth::info::auth::AuthContext;

use port::app::ctx::AppContext;
use service::cola_user::user::state::UserStateService;
use crate::model::vo::comment::CommentListResponse;
////////

/// # [GET API] - 评论 获取
pub struct CommentGetApi;

impl CommentGetApi {
    //

    ////////

    /// # 1. [API HANDLER] - 视频的评论
    pub async fn api_get_video_list(
        auth: AuthContext,
        video_id: Option<i64>,
        query: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<CommentListResponse> {
        // 1. 参数校验：获取 uid
        let uid = auth.uid;

        // 2. 参数校验：video_id
        let video_id = match video_id {
            Some(id) => id,
            None => {
                return AppData::err(4002, "缺少video_id参数", None);
            }
        };

        // 3. 主流程
        match CommentGetCase::case_get_video_comments_list(uid, video_id, query, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Comment error: {:?}", e);
                AppData::err(5001, "获取视频的评论列表失败", None)
            }
        }
    }

    ////////

    /// # 2. [API HANDLER] - 查看用户的评论
    pub async fn api_get_user_list(
        auth: AuthContext,
        _url: ApiGatewayRequest,
        query: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> AppData<CommentListResponse> {
        let uid = auth.uid;

        // 2. Call Service 检查用户状态
        if let Err(_) = UserStateService::check_user_valid(uid).await {
            return AppData::err(4004, "用户不存在或者已被删除", None);
        }

        // 3. 主流程 —— 🌟 核心修复：Some(uid) 改成 uid，并在末尾追加 ctx
        match CommentGetCase::case_get_user_comments_list(uid, query, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Comment error: {:?}", e);
                AppData::err(5001, "获取用户的评论列表失败", None)
            }
        }
    }
}

//////// END
