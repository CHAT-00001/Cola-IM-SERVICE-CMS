// cola_video/src/case/danmaku/add.rs -- VIDEO - CASE - 弹幕发布
// 2026/9/7 Created.

////////

use crate::assembler::danmaku::build_danmaku_single_response;
use crate::model::vo::danmaku::DanmakuSingleResponse;
use anyhow::Result;
use cola_data::cola_fs::rick_check;
use cola_data::cola_video::command::danmaku::DanmakuCommand;
use port::app::ctx::AppContext;
use tracing::info;

////////

/// # [CASE] - 视频弹幕发布用例
pub struct DanmakuAddCase;

impl DanmakuAddCase {
    //

    ////////

    /// # 1. [CASE] - 发送弹幕
    /// * `desc`: 风控后调用弹幕 Port 持久化，并组装单条响应
    pub async fn case_send(
        uid: i64,            // 操作者 ID
        video_id: i64,       // 视频 ID
        cmd: DanmakuCommand, // 弹幕命令
        ctx: AppContext,     // 应用上下文
    ) -> Result<DanmakuSingleResponse> {
        // ⚠️ 风控检查
        let check_text = format!("{} {:?}", cmd.content, cmd.at);
        let visibility = rick_check(check_text).await;

        // 构造响应体
        let info = ctx
            .video
            .danmaku
            .add
            .send_danmaku(uid, video_id, cmd, visibility)
            .await
            .map_err(|error| anyhow::anyhow!("弹幕发布持久化失败: {error}"))?;
        info!("[🗣️ ADD CASE] - ✅️ 弹幕发布成功: uid={uid}, video_id={video_id}");
        build_danmaku_single_response(info, Some(uid), 0).await
    }
}

//////// END
