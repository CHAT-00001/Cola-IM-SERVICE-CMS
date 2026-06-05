// cola_video/src/live/biz/video_response  -- 视频浏览 逻辑
// 2026-03-30 08:25 (Aligned with ViewPort)

////////

use std::sync::Arc;
use crate::video::port::view::ViewPort;
use anyhow::{Context, Result};
use data::video::entity::video::VideoEntity;
use data::video::info::video::VideoInfo;
use data::video::model::video::VideoSingleResponse;
use crate::user::port::user::UserPort;
use crate::video::assembler::video::build_video_single_response;
////////

/// # [LOGIC] - 获取视频详情
// 修改 logic 层，去掉 port 参数
pub async fn logic_get_video_detail(
    user_id: i64,
    video_id: i64,
) -> Result<VideoSingleResponse> {
    // 直接在函数内部创建 adapter，不需要外部传参
    let video_port = get_video_port();
    let user_port = get_user_port();
    let music_port = get_music_port();

    // 查询视频信息
    let video_info :VideoInfo = video_port
        .find_info_by_id(video_id)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 查询视频详情失败: {}", e))?;

    // 如果需要记录观看行为，取消注释
    // let _ = view_port
    //     .record_view_behavior(cola_data.user_id, video_id)
    //     .await;

    // 构建响应
    let resp = build_video_single_response(video_info , Some(user_id),&*user_port).await?;

    Ok(resp)
}

/// # LOGIC
/// # 更新播放状态 / 记录播放进度
pub async fn logic_change_play_status(
    query: crate::video::app::view::ViewQuery,
    view_port: &dyn ViewPort,
) -> Result<bool> {
    // 1. 如果 uid 是 None，说明未登录，直接返回 Ok(true) 跳过后续逻辑
    let Some(uid) = query.uid else {
        return Ok(true);
    };

    // 2. 此时 uid 已经被解包为具体的类型（如 i64/u32）
    if let Some(pos) = query.play_pos {
        if pos > 0 {
            view_port
                .update_play_progress(uid, query.video_id, pos as f32)
                .await
                .map_err(|e| anyhow::anyhow!("BIZ: 更新播放进度失败: {}", e))?;
        }
    }

    Ok(true)
}

/// # LOGIC
/// # 投稿/创建视频
pub async fn logic_create_video(
    entity: VideoEntity,
    view_port: &dyn ViewPort,
) -> Result<VideoEntity> {
    view_port
        .create_new_video(entity)
        .await
        .map_err(|e| anyhow::anyhow!("BIZ: 创建视频失败: {}", e))
}
