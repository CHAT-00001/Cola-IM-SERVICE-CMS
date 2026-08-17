// cola_video/case/view/get.rs
// 视频 - case - 浏览 - 获取
// 2026/8/4 20:21 Created.

////////

use crate::assembler::video::build_video_single_response_with_cdn;
use crate::case::storage::resolve_video_cdn_domain;
use anyhow::{Result, anyhow};
use port::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::app::response::ListResponse;
use cola_data::cola_video::info::video::{VideoInfo, VideoSingleResponse};
use cola_data::cola_video::info::view::VideoViewInfo;

////////

/// # [ADD CASE] - 浏览 用例
/// * `desc`: `视频浏览发布用例`
pub struct VideoViewAddCase;

impl VideoViewAddCase {
    //

    ////////

    /// # 1. [USE CASE] - 我的
    /// * `desc`: `获取我的浏览列表`
    pub async fn case_get_my_views_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<()> {
        // Call Service Port
        ctx.video
            .view
            .get_my_viewed_list(uid, url.limit, url.offset)
            .await
            .map_err(|e| anyhow!("[🤐 CASE]: ❌️ 获取我浏览过的视频列表失败: {}", e))?;
        Ok(())
    }

    ////////

    /// # 2. [USE CASE] - TA的
    // * `desc`: `获取TA的浏览列表`
    pub async fn case_get_here_views_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<VideoSingleResponse> {
        // Call Service Port
        let info: VideoInfo = ctx
            .video
            .view
            .get_video_list_by_id(url.video_id)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 CASE]: ❌️ 获取TA浏览过的视频列表失败: {}", e))?;
        // 构建响应
        let cdn_domain = resolve_video_cdn_domain(ctx, "short-video").await?;
        let resp = build_video_single_response_with_cdn(info, Some(uid), &cdn_domain).await?;

        Ok(resp)
    }

    ////////

    /// # 3. [USE CASE] - 视频
    // * `desc`: `获取浏览ID的浏览记录列表`
    pub async fn case_get_video_views_list(
        uid: i64,
        url: ApiGatewayRequest,
        ctx: &AppContext,
    ) -> Result<ListResponse> {
        // Call Service Port
        let infos: VideoViewInfo = ctx
            .video
            .view
            .get_video_list_by_id(url.video_id)
            .await
            .map_err(|e| anyhow::anyhow!("[🤐 CASE]: ❌️ 获取视频ID的浏览记录列表失败: {}", e))?;
        // 构建响应
        let resp = build_video_single_response(infos, Some(uid)).await?;

        Ok(resp)
    }
}

//////// END
