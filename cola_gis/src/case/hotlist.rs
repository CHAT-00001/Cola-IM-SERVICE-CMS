// cola_gis/src/case/hotlist.rs  -- 可乐GIS - 用例层 - 上热门 - mod
// 2026-07-07 10:20

////////

use anyhow::{Result, anyhow};
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_gis::command::hotlist::PoiHotlistCommand;
use port::app::ctx::AppContext;
use tracing::{info, warn};

////////

/// # [CASE] - 上热门 用例
pub struct HotlistCase;

impl HotlistCase {
    //

    ////////

    /// # 1. [CASE] - 添加
    pub async fn case_add_hotlist(
        uid: i64,
        url: ApiGatewayRequest,
        cmd: PoiHotlistCommand,
        ctx: &AppContext,
    ) -> Result<()> {
        // 1. 保存上热门(抖+)记录
        ctx.gis
            .hotlist
            .save_hotlist_record(uid, url.video_id, cmd)
            .await
            .map_err(|e| anyhow!("检查兴趣点健康状态失败: {}", e))?;

        Ok(())
    }

    ////////
}

////// END
