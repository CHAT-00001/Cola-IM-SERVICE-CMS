// usecase/src/live/app/like.rs  -- 互动接口
// 2026-01-20 10:11

// usecase/src/live/app/interact.rs

// usecase/src/live/app/interact.rs

use data::app::data::AppData;
use crate::video::biz;
use serde::{Deserialize, Serialize};
use cola_data::app::data::AppData;


/// ## 互动请求参数
#[derive(Deserialize, Debug)]
pub struct InteractRequest {
    pub uid: i64,         // 操作人 ID
    pub target_id: i64,   // 目标对象 ID (视频ID)
    pub action_type: i16, // 动作类型/举报类型
}

// 内部辅助函数：统一处理 Biz 到 AppData 的转换
async fn wrap_result(res: anyhow::Result<()>, err_code: i32) -> AppData<()> {
    match res {
        Ok(_) => AppData::ok(()),
        Err(e) => AppData::err(err_code, e.to_string(), None),
    }
}

// --- UseCase 业务编排层 ---

/// ## 添加点赞
pub async fn add_like(uid: i64, video_id: i64, port: &dyn LikePort) -> AppData<()> {
    wrap_result(
        biz::like::add_like_logic(uid, video_id, port).await,
        5001,
    )
    .await
}

/// ## 取消点赞
pub async fn delete_like(uid: i64, video_id: i64, port: &dyn LikePort) -> AppData<()> {
    wrap_result(
        biz::like::remove_like_logic(uid, video_id, port).await,
        5002,
    )
    .await
}

/// ## 收藏视频
pub async fn add_collect(uid: i64, video_id: i64, port: &dyn LikePort) -> AppData<()> {
    wrap_result(
        biz::like::add_collect_logic(uid, video_id, port).await,
        5003,
    )
    .await
}

/// ## 移除收藏
pub async fn delete_collect(uid: i64, video_id: i64, port: &dyn LikePort) -> AppData<()> {
    wrap_result(
        biz::like::remove_collect_logic(uid, video_id, port).await,
        5004,
    )
    .await
}

/// ## 增加浏览次数
pub async fn add_view(video_id: i64, port: &dyn LikePort) -> AppData<()> {
    wrap_result(
        biz::like::increment_view_logic(video_id, port).await,
        5005,
    )
    .await
}

/// ## 报告完播
pub async fn report_play_done(uid: i64, video_id: i64, port: &dyn LikePort) -> AppData<()> {
    wrap_result(
        biz::like::mark_done_logic(uid, video_id, port).await,
        5006,
    )
    .await
}

/// ## 推上热门 (管理接口)
pub async fn push_to_hot(video_id: i64, port: &dyn LikePort) -> AppData<()> {
    wrap_result(biz::like::set_hot_logic(video_id, port).await, 5007).await
}

/// ## 举报视频
pub async fn report_video(req: InteractRequest, port: &dyn LikePort) -> AppData<()> {
    wrap_result(
        biz::like::report_logic(req.uid, req.target_id, req.action_type, port).await,
        5008,
    )
    .await
}
