// usecase/src/live/app/video_response
// 2026-03-30 07:55 (Optimized)

////////

use crate::ctx::AppContext;
use crate::user::port::user::UserPort;
use crate::video::assembler::video::build_video_single_response;
use crate::video::biz;
use crate::video::port::view::ViewPort;
use data::app::data::AppData;
use data::video::model::video::VideoSingleResponse;
use sqlx::query;
////////

/// # QUERY
#[derive(Debug, Clone)]
pub struct ViewQuery {
    pub uid: Option<i64>,
    pub video_id: i64,
    pub play_pos: Option<i32>,
}

impl ViewQuery {
    pub fn new(video_id: i64, uid: Option<i64>) -> Self {
        Self {
            uid,      // 核心修改：加上 uid:
            video_id, // 变量名与字段名一致，允许缩写
            play_pos: None,
        }
    }
}

////////

/// # [CASE] - 查看视频详情
/// * 1001
pub async fn case_get_video_detail(query: ViewQuery) -> AppData<VideoSingleResponse> {
    let user_id = query.uid.unwrap_or(0);

    match biz::view::logic_get_video_detail(user_id, query.video_id).await {
        Ok(resp) => AppData::ok(resp),
        Err(e) => {
            tracing::error!("VIDEO_DETAIL_ERROR: {:?}", e);
            AppData::err(5001, "APP: 获取视频详情失败", None)
        }
    }
}

////////

/// # APP
/// # 修改播放状态/记录进度
pub async fn api_change_play_status(query: ViewQuery, view_port: &dyn ViewPort) -> AppData<bool> {
    match biz::view::logic_change_play_status(query, view_port).await {
        Ok(_) => AppData::ok(true),
        Err(e) => {
            tracing::error!("DEBUG - Change Status Error: {:?}", e);
            AppData::err(5002, "APP: 更新播放进度失败", None)
        }
    }
}
