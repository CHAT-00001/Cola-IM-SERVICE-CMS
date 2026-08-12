// cola_gis/src/case/add  -- GIS - 用例层 - 分享
// 2026/6/10 08:14

////////

use anyhow::{Result, anyhow};
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_video::command::share::ShareCommand;
use port::app::ctx::AppContext;
use tracing::{error, info, warn};

////////

/// # [APP USE CASE] - 分享用例
pub struct ShareCase;

impl ShareCase {
    //

    ////////

    /// # 1. [CASE] - 添加分享
    pub async fn case_add_video_share(
        uid: i64,
        url: ApiGatewayRequest,
        cmd: ShareCommand,
        ctx: &AppContext,
    ) -> Result<()> {
        // Call ..
        ctx.video
            .share
            .add
            .save_share_record(uid, url.video_id, cmd)
            .await
            .map_err(|e| anyhow!("发布分享视频记录失败:  {}", e))?;
        error!("[🤐 CASE] - ❌️ 发布分享记录失败: ID: {}", url.id);

        Ok(())
    }

    /// # 2. [CASE] - 移除分享
    pub async fn case_remove_collect_share(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<()> {
        // Call ..
        ctx.video
            .share
            .del
            .delete_share_record(uid, url.id)
            .await
            .map_err(|e| anyhow!("删除分享记录失败: {}", e))?;
        error!("[🤐 CASE] - ❌️ 删除分享记录失败: ID: {}", url.id);
        Ok(())
    }
}

//////// END
