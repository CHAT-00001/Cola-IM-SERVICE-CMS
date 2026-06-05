// cola_video/src/video/app/danmaku  -- VIDEO - 应用层 - 弹幕
// 2026/4/24 18:51

////////

use cola_data::app::data;
use cola_data::app::data::AppData;
use crate::biz;
use crate::model::vo::danmaku::DanmakuSingleResponse;

////////

/// # 1. [CASE] - 发布弹幕
/// 注意：该方法已重定向到 `case_add_danmaku` (带认证版本)
pub async fn case_add_danmaku_old(
    auth: &data::auth::info::auth::AuthContext,
    query: &data::video::command::danmaku::DanmakuCommand,
) -> AppData<DanmakuSingleResponse> {
    let uid = auth.user_id.unwrap_or(0);

    match biz::danmaku::logic_add_danmaku(
        uid,
        query.video_id,
        query.content.clone(),
        query.play_time,
    )
    .await
    {
        Ok(entity) => AppData::ok(DanmakuSingleResponse {
            info: Default::default(),
        }),
        Err(e) => {
            tracing::error!("Add Danmaku Error: {:?}", e);
            AppData::err(5001, "发送弹幕失败", None)
        }
    }
}

//////

/// # 2. [CASE] - 获取弹幕列表
pub async fn case_get_danmaku(
    user_id: i64,
    video_id: i64,
    play_time: i32,
) -> AppData<String> {
    let segment_size = 5000;
    match biz::danmaku::logic_get_danmaku_segment(
        user_id,
        video_id,
        play_time,
        play_time + segment_size,
        20,
    )
    .await
    {
        Ok(list) => AppData::ok(list),
        Err(e) => {
            tracing::error!("Get Danmaku Error: {:?}", e);
            AppData::err(5002, "获取弹幕列表失败", None)
        }
    }
}


//////// END