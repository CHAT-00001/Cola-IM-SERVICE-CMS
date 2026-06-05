// cola_video/src/assembler/danmaku.rs  -- VIDEO - 组装 - 组装弹幕响应体
// 2026/06/05 10:10 by wx: cestbon10080

use anyhow::Result;
use std::collections::HashMap;

use cola_data::app::page::PageInfo;
use cola_data::user::info::user::UserInfo;
use cola_data::video::entity::danmaku::DanmakuEntity;
use crate::model::info::danmaku::DanmakuInfo;
use crate::model::vo::danmaku::{DanmakuListResponse, DanmakuSingleResponse, DanmakuVo};
use repo::user::service::user::UserService; // 🚀 引入静态 UserService

////////

/// # [BUILD] - 组装单弹幕响应
pub async fn build_danmaku_single_response(
    entity: DanmakuEntity,
    current_uid: Option<i64>,
    video_author_id: i64,
) -> Result<DanmakuSingleResponse> {

    // 1. 静态调用：获取发送者用户信息
    let sender_user_info = UserService::find_user_info_by_id(entity.user_id).await?;

    // 2. 组装 Info
    let danmaku_info = DanmakuInfo::from_entity(entity, sender_user_info, video_author_id);

    // 3. 组装 VO
    let danmaku_vo = DanmakuVo::from_info(
        danmaku_info,
        current_uid,
        video_author_id,
        false, // is_liked
        false, // is_disliked
    );

    Ok(DanmakuSingleResponse { info: danmaku_vo })
}

////////

/// # [BUILD] - 组装多弹幕列表响应
pub async fn build_danmaku_list_response(
    entities: Vec<DanmakuEntity>,
    current_uid: Option<i64>,
    video_author_id: i64,
    page: i64,
    qty: i64,
    total: i64,
) -> Result<DanmakuListResponse> {

    // 1. 静态调用：批量获取用户信息 (UserService 保证全量填充)
    let author_ids: Vec<i64> = entities.iter().map(|e| e.user_id).collect();
    let authors_map = UserService::find_user_info_by_uids(&author_ids).await?;

    // 2. 迭代组装
    let danmakus: Vec<DanmakuVo> = entities
        .into_iter()
        .map(|entity| {
            let sender_user_info = authors_map.get(&entity.user_id).cloned().unwrap_or_default();

            let danmaku_info = DanmakuInfo::from_entity(entity, sender_user_info, video_author_id);

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