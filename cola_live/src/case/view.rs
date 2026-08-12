// cola_live/src/case/get
// core - LIVE - case - view 浏览用例
// 2026-03-30 08:25

////////

use crate::api::view;
use crate::assembler::video::build_video_single_response;
use crate::model::vo::video::VideoSingleResponse;
use anyhow::{Result, anyhow};
use port::app::ctx::AppContext;
use cola_data::cola_video::info::video::VideoInfo;
use std::sync::Arc;
use cola_data::app::query::ApiGatewayRequest;

////////

/// # [USE CASE] - 浏览 用例
pub struct ViewCase;

impl ViewCase {
    //
    ////////

    /// # [CASE] - 保存
    /// * `params` interact
    pub async fn case_add_video_view(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext) -> Result<()> {

        // Call Service Port
        ctx.video
            .view
            .add
            .save_view(uid, url.video_id)
            .await
            .map_err(|e| anyhow!("保存浏览记录 + 更新浏览数量失败: {}", e))?;
        Ok(())
    }

    ////////

    /// # [CASE] - 获取
    // 修改 logic 层，去掉 port 参数
    pub async fn case_get_video_detail(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoSingleResponse> {

        // Call Service Port
        let info: VideoInfo = ctx
            .video
            .view
            .get
            .get_video_info_by_id(url.video_id)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 查询视频详情失败: {}", e))?;

        // 如果需要记录观看行为，取消注释
        // let _ = view_port
        //     .record_view_behavior(cola_data.user_id, video_id)
        //     .await;

        // 构建响应
        let resp = build_video_single_response(info, Some(uid)).await?;

        Ok(resp)
    }
}

//////// END
