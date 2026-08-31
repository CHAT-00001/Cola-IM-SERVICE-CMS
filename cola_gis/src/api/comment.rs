// cola_gis/src/api/add  -- 可乐GIS - 应用层 - 评论
// 2026-07-07

////////

use crate::case::comment::CommentCase;
use crate::model::vo::poi_comment::{CommentListResponse, CommentSingleResponse};
use cola_data::app::data::AppData;
use cola_data::app::request::ApiUrlParamsQuery;
use cola_data::auth::info::auth::AuthContext;
use cola_data::cola_gis::command::comment::PoiCommentCommand;
use port::app::ctx::AppContext;

////////
pub struct CommentParamsQuery {
    pub poi_id: i64,
    pub comment_id: i64,
    pub is_unliked: bool,
    pub snowflake: i64,
}

//////
pub struct CommentApi;

impl CommentApi {
    ////////

    /// # 1. [API HANDLER] - 发布评论
    pub async fn handler_add_comment(
        auth: AuthContext,
        url: CommentParamsQuery,
        cmd: PoiCommentCommand,
        ctx: &AppContext,
    ) -> AppData<CommentSingleResponse> {
        let poi_id = url.poi_id;

        match CommentCase::case_add_comment(auth.uid, poi_id, cmd, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("New Comment Error: {:?}", e);
                AppData::err(5001, "发布评论失败", None)
            }
        }
    }

    ////////

    /// # 2. [API HANDLER] - 兴趣点的评论列表
    pub async fn handler_view_poi_list(
        auth: AuthContext,
        poi_id: Option<i64>,
        query: ApiUrlParamsQuery,
        ctx: &AppContext,
    ) -> AppData<CommentListResponse> {
        let uid = auth.uid;

        let poi_id = match poi_id {
            Some(id) => id,
            None => {
                return AppData::err(4002, "缺少poi_id参数", None);
            }
        };

        match CommentCase::case_get_poi_comments_list(uid, poi_id, query, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Comment error: {:?}", e);
                AppData::err(5001, "获取POI的评论列表失败", None)
            }
        }
    }

    ////////

    /// # 4. [API HANDLER] - 单条删除
    pub async fn handler_del_comment(
        auth: AuthContext,
        url: CommentParamsQuery,
        ctx: &AppContext,
    ) -> AppData<String> {
        let comment_id = match url.comment_id {
            0 => {
                return AppData::err(4002, "缺少comment_id参数", None);
            }
            id => id,
        };

        match CommentCase::case_del_one_item(auth.uid, comment_id, ctx).await {
            Ok(resp) => AppData::ok(resp),
            Err(e) => {
                tracing::error!("Comment error: {:?}", e);
                AppData::err(6001, "评论删除失败", None)
            }
        }
    }

    ////////
}

////// END
