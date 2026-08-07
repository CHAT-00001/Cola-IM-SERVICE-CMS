// cola_gis/src/case/add  -- 用例层 - 上热门
// 2026-07-07

//////

use anyhow::{Result, anyhow};
use tracing::{info, warn};
use cola_data::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_gis::command::hotlist::HotlistCommand;

//////

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
        ctx.gis.hotlist
            .save_hotlist_record(uid, url.video_id, cmd)
            .await
            .map_err(|e| anyhow!("检查兴趣点健康状态失败: {}", e))?;

        Ok(())
    }

    ////////

}

////// END