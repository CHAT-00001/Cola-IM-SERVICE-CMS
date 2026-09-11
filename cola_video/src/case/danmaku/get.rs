// cola_video/src/case/danmaku/get.rs -- VIDEO - case - 弹幕 - 获取用例
// 2026/9/7 15:26 Created.

////////

use crate::assembler::danmaku::build_danmaku_list_response;
use crate::model::vo::danmaku::DanmakuListResponse;
use anyhow::Result;
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [ADD CASE] - 视频弹幕发布用例
pub struct DanmakuGetCase;

impl DanmakuGetCase {
    //

    ////////

    /// # 1. [CASE] - 最新
    pub async fn case_new(
        uid: i64,        // 操作者 ID
        video_id: i64,   // 视频 ID
        play_time: i32,  // 播放时间
        qty: i32,        // 数量
        ctx: AppContext, // 应用上下文
    ) -> Result<DanmakuListResponse, anyhow::Error> {
        // 2. Call Port.. 装载弹幕信息列表
        let (danmaku_infos, total) = ctx
            .video
            .danmaku
            .list
            .get_danmaku_by_video_id(uid, video_id, play_time, qty)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 ADD CASE]: ❌️ 获取最新的弹幕列表失败: {}", e))?;

        info!(
            "[🗣️ ADD CASE] - ✅️ 获取最新的弹幕列表成功: uid={}, play_time={}",
            uid, play_time
        );

        // 3. Call Assembler .. 组装成弹幕Vo
        let response = build_danmaku_list_response(
            danmaku_infos,
            Some(uid),
            0,
            1,
            qty as i64,
            total,
        )
        .await?;

        Ok(response)
    }
}

//////// END
