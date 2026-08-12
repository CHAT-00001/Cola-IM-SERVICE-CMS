// cola_live/src/case/danmaku/add.rs
//  LIVE - 用例层 - 弹幕 - 发布
// 2026/8/12 04:42 Created.

////////

use anyhow::Result;
use cola_data::cola_video::command::danmaku::DanmakuCommand;
use port::app::ctx::AppContext;
use service::cola_video::danmaku::add::VideoDanmakuAddService;

////////

/// # [CASE] - 弹幕 发布
/// * `DESC`: `LIVE` - `弹幕发布用例编排`
pub struct DanmakuAddCase;

////////
impl DanmakuAddCase {
    //

    ////////

    /// # 1. [CASE] - 发表弹幕
    /// * `描述` 用户UGC弹幕应用编排
    pub async fn case_add_danmaku(uid: i64, video_id: i64, cmd: DanmakuCommand) -> Result<String> {
        // 1. 检查弹幕内容风控等级
        let key = cmd.content.to_string();
        let visibility = 5;

        // 2. 调用SERVICE
        let service =
            VideoDanmakuAddService::save_danmaku_and_update_count(uid, video_id, cmd, visibility)
                .await?;

        // 3. 返回成功给API层响应
        Ok("ok".to_string())
    }
}

//////// END
