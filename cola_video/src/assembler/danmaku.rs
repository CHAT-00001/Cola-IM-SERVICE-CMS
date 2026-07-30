// cola_video/src/assembler/danmaku.rs  -- VIDEO - 组装 - 组装弹幕响应体
// 2026/06/05 10:10

////////

use anyhow::Result;
use cola_data::app::page::PageInfo;
use cola_data::video::info::danmaku::DanmakuInfo;
use crate::model::vo::danmaku::{DanmakuListResponse, DanmakuSingleResponse, DanmakuVo};
use repository::user::service::user::UserService;

////////

/// # [BUILD] - 组装单弹幕响应
pub async fn build_danmaku_single_response(
    danmaku_info: DanmakuInfo, // 🚀 升级：直接吃 Info
    current_uid: Option<i64>,
    video_author_id: i64,
) -> Result<DanmakuSingleResponse> {

    let _sender_user_info = UserService::get_user_info_by_id(danmaku_info.user_id).await?;

    let is_liked = false;
    let is_disliked = false;

    // 组装 VO
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
    infos: Vec<DanmakuInfo>, // 🚀 核心重构：全面改收 Vec<DanmakuInfo>，跟 Repo 对齐！
    current_uid: Option<i64>,
    video_author_id: i64,
    page: i64,
    qty: i64,
    total: i64,
) -> Result<DanmakuListResponse> {

    // 1. 静态调用：批量获取用户信息
    let author_ids: Vec<i64> = infos.iter().map(|i| i.user_id).collect();
    let _authors_map = UserService::get_user_info_by_ids(&author_ids).await?;

    // 2. 迭代组装
    let danmakus: Vec<DanmakuVo> = infos
        .into_iter()
        .map(|danmaku_info| {
            // 通过 VO 的顶级流控函数，去计算 is_own, is_author 等复杂身份状态
            DanmakuVo::from_info(
                danmaku_info,
                current_uid,
                video_author_id,
                false,
                false,
            )
        })
        .collect();

    // 3. 分页逻辑
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