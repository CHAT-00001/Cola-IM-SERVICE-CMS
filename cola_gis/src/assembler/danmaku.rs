// cola_gis/src/assembler/danmaku.rs  -- GIS - 组装 - 组装弹幕响应体
// 2026-07-07

////////

use anyhow::Result;
use cola_data::app::page::PageInfo;
use cola_data::gis::info::danmaku::PoiDanmakuInfo;
use crate::model::vo::poi_danmaku::{DanmakuListResponse, DanmakuSingleResponse, DanmakuVo};
use repository::user::service::user::UserService;

////////

/// # [BUILD] - 组装单弹幕响应
pub async fn build_danmaku_single_response(
    danmaku_info: PoiDanmakuInfo,
    current_uid: Option<i64>,
    video_author_id: i64,
) -> Result<DanmakuSingleResponse> {

    let _sender_user_info = UserService::get_user_info_by_id(danmaku_info.user_id).await?;

    let is_liked = false;
    let is_disliked = false;

    let danmaku_vo = DanmakuVo::from_info(
        danmaku_info,
        current_uid,
        video_author_id,
        is_liked,
        is_disliked,
    );

    Ok(DanmakuSingleResponse { info: danmaku_vo })
}

////////

/// # [BUILD] - 组装多弹幕列表响应
pub async fn build_danmaku_list_response(
    infos: Vec<PoiDanmakuInfo>,
    current_uid: Option<i64>,
    video_author_id: i64,
    page: i64,
    qty: i64,
    total: i64,
) -> Result<DanmakuListResponse> {

    let author_ids: Vec<i64> = infos.iter().map(|i| i.user_id).collect();
    let _authors_map = UserService::get_user_info_by_ids(&author_ids).await?;

    let danmakus: Vec<DanmakuVo> = infos
        .into_iter()
        .map(|danmaku_info| {
            DanmakuVo::from_info(
                danmaku_info,
                current_uid,
                video_author_id,
                false,
                false,
            )
        })
        .collect();

    Ok(DanmakuListResponse {
        danmakus,
        page_info: PageInfo {
            page,
            qty,
            has_more: (page * qty) < total,
        },
    })
}

//////// END