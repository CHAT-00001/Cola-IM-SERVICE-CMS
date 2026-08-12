// cola_video/src/case/add  -- VIDEO - 用例层 - 上热门
// 2026/6/10 08:37

////////

use anyhow::{Result, anyhow};
use tracing::{info, warn};
use port::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_video::command::hotlist::HotlistCommand;


////////

/// # [CASE] - 上热门 用例
pub struct HotlistCase;

impl HotlistCase {

    ////////

    /// # 1. [CASE] - 添加
    pub async fn case_add_hotlist(
        uid: i64,
        url: ApiGatewayRequest,
        cmd: HotlistCommand,
        ctx: &AppContext,
    ) -> Result<()> {

        // 1. 保存上热门(抖+)记录
        ctx.video
            .hotlist
            .add
            .save_hotlist(uid, url.video_id, cmd)
            .await
            .map_err(|e| anyhow!("检查视频健康状态失败: {}", e))?;

        Ok(())
    }

    ////////

}

//////// END
