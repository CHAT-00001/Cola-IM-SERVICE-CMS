// cola_video/src/assembler/comment.rs  -- VIDEO - 组装 - 评论响应体
// 2026/06/05 09:50 Created.

////////

use anyhow::Result;
use cola_data::app::page::PageInfo;
use cola_data::cola_user::info::user::UserInfo;
use cola_data::cola_user::vo::user::UserVo;
use cola_data::cola_video::info::comment::CommentInfo;
use cola_data::cola_video::vo::comment::{CommentListResponse, CommentSingleResponse, CommentVo};
use cola_user::case::user::avatar_cdn::{resolve_avatar_cdn_domain, resolve_avatar_url};
use port::app::ctx::AppContext;
use service::cola_user::user::active::UserService;
use std::collections::{HashMap, HashSet};

////////

/// # 1. [ASSEMBLER] - 组装单评论响应
/// * `desc`: 动态加载评论作者、视频作者和当前用户互动关系
pub async fn build_comment_single_response(
    comment_info: CommentInfo, // 评论 Info
    current_uid: Option<i64>,  // 当前用户 ID
    ctx: &AppContext,          // 应用上下文
) -> Result<CommentSingleResponse> {
    let current_uid = current_uid.unwrap_or_default();
    let user_info = UserService::get_user_info_by_id(comment_info.user_id).await?;
    let video_info = ctx
        .video
        .video
        .get
        .get_video_info_by_id(current_uid, comment_info.video_id)
        .await?;
    let is_like = if current_uid > 0 {
        ctx.video
            .comment
            .like
            .check_state(current_uid, comment_info.id)
            .await?
    } else {
        false
    };
    let is_dislike = if current_uid > 0 {
        ctx.video
            .comment
            .dislike
            .check_state(current_uid, comment_info.id)
            .await?
    } else {
        false
    };

    let mut user_info = user_info;
    user_info.avatar_url =
        resolve_avatar_url(&user_info.avatar_url, &resolve_avatar_cdn_domain(ctx).await);
    let comment_vo = CommentVo::new(
        comment_info.clone(),
        build_user_vo(user_info),
        comment_info.user_id == video_info.uid,
        comment_info.user_id == current_uid,
        is_like,
        is_dislike,
    );

    Ok(CommentSingleResponse { info: comment_vo })
}

////////

/// # 2. [ASSEMBLER] - 组装评论列表
/// * `desc`: 批量加载用户信息，并逐条动态检查作者归属和互动关系
pub async fn build_comment_list_response(
    infos: Vec<CommentInfo>,  // 评论 Info 列表
    current_uid: Option<i64>, // 当前用户 ID
    page: i64,                // 页码
    qty: i64,                 // 分页数量
    total: i64,               // 总数量
    ctx: &AppContext,         // 应用上下文
) -> Result<CommentListResponse> {
    if infos.is_empty() {
        return Ok(CommentListResponse {
            list: Vec::new(),
            page: PageInfo {
                page,
                qty,
                has_more: false,
            },
        });
    }

    let current_uid = current_uid.unwrap_or_default();
    let user_ids: Vec<i64> = infos
        .iter()
        .map(|info| info.user_id)
        .filter(|user_id| *user_id > 0)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let users: HashMap<i64, UserInfo> = UserService::get_user_info_by_ids(&user_ids).await?;
    let video_ids: Vec<i64> = infos
        .iter()
        .map(|info| info.video_id)
        .filter(|video_id| *video_id > 0)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    let videos = ctx
        .video
        .video
        .get
        .get_video_infos_by_ids(current_uid, video_ids)
        .await?
        .into_iter()
        .map(|video| (video.id, video.uid))
        .collect::<HashMap<_, _>>();
    let avatar_cdn_domain = resolve_avatar_cdn_domain(ctx).await;

    let mut list = Vec::with_capacity(infos.len());
    for comment_info in infos {
        let user_info = users
            .get(&comment_info.user_id)
            .cloned()
            .unwrap_or_default();
        let is_like = if current_uid > 0 {
            ctx.video
                .comment
                .like
                .check_state(current_uid, comment_info.id)
                .await?
        } else {
            false
        };
        let is_dislike = if current_uid > 0 {
            ctx.video
                .comment
                .dislike
                .check_state(current_uid, comment_info.id)
                .await?
        } else {
            false
        };
        let is_author = videos
            .get(&comment_info.video_id)
            .is_some_and(|video_author_id| *video_author_id == comment_info.user_id);

        let mut user_info = user_info;
        user_info.avatar_url = resolve_avatar_url(&user_info.avatar_url, &avatar_cdn_domain);
        list.push(CommentVo::new(
            comment_info.clone(),
            build_user_vo(user_info),
            is_author,
            comment_info.user_id == current_uid,
            is_like,
            is_dislike,
        ));
    }

    Ok(CommentListResponse {
        list,
        page: PageInfo {
            page,
            qty,
            has_more: (page * qty) < total,
        },
    })
}

////////

/// # 3. [ASSEMBLER] - 构造用户视图
/// * `desc`: 使用查询得到的 UserInfo 动态组装 UserVo
fn build_user_vo(user_info: UserInfo) -> UserVo {
    UserVo::new(
        user_info.clone(),
        user_info.is_following,
        user_info.is_online,
        user_info.is_streaming,
    )
}

//////// END
