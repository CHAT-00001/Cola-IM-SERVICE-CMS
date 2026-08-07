// cola_video/case/view/add.rs
// 视频 - case - 浏览 - 发布
// 2026/8/4 20:18 Created.

////////

use crate::assembler::video::build_video_single_response;
use anyhow::{Result, anyhow};
use cola_data::app::ctx::AppContext;
use cola_data::app::query::ApiGatewayRequest;
use cola_data::cola_video::info::video::{VideoInfo, VideoSingleResponse};
use std::sync::Arc;
use tracing::{error, info};

////////

/// # [ADD USE CASE] - 浏览 用例
/// * `desc`: `视频浏览发布用例`
pub struct VideoViewAddCase;

impl VideoViewAddCase {
    //

    ////////

    /// # 1. [USE CASE] - 发布
    /// * `desc`:  `视频浏览发布用例`
    pub async fn case_add_view(
        uid: i64,               // 操作者ID
        url: ApiGatewayRequest, // 网关参数
        ctx: &AppContext,       // 全局上下文
    ) -> Result<()> {
        info!(
            "[🔍 CASE] - 🚀 开始执行【发布浏览记录】用例: uid = {}, video_id = {}",
            uid, url.video_id
        );

        // Call Service Port
        let result = ctx
            .video
            .view
            .save_view_record_update_views_count(uid, url.video_id)
            .await;

        match result {
            Ok(_) => {
                info!(
                    "[🔍 CASE] - ✅ 【发布浏览记录】成功: uid = {}, video_id = {}",
                    uid, url.video_id
                );
                Ok(())
            }
            Err(e) => {
                error!(
                    "[🤐 CASE] - ❌️ 发布浏览记录 + 更新浏览数量失败: uid = {}, video_id = {}, err = {}",
                    uid, url.video_id, e
                );
                Err(anyhow!(
                    "[🤐 CASE] - ❌️ 发布浏览记录 + 更新浏览数量失败: {}",
                    e
                ))
            }
        }
    }

    ////////

    /// # 2. [USE CASE] - 更新
    /// * `desc`:  `视频浏览更新用例`
    pub async fn case_update_view(
        uid: i64,               // 操作者ID
        url: ApiGatewayRequest, // 网关参数
        ctx: &AppContext,       // 全局上下文
    ) -> Result<()> {
        info!(
            "[🔍 CASE] - 🔄 开始执行【更新浏览记录】用例: uid = {}, video_id = {}",
            uid, url.video_id
        );

        // Call Service Port
        let result = ctx
            .video
            .view
            .save_view_record_update_views_count(uid, url.video_id)
            .await;

        match result {
            Ok(_) => {
                info!(
                    "[🔍 CASE] - ✅ 【更新浏览记录】成功: uid = {}, video_id = {}",
                    uid, url.video_id
                );
                Ok(())
            }
            Err(e) => {
                error!(
                    "[🤐 CASE] - ❌️ 更新浏览记录 + 更新浏览数量失败: uid = {}, video_id = {}, err = {}",
                    uid, url.video_id, e
                );
                Err(anyhow!(
                    "[🤐 CASE] - ❌️ 更新浏览记录 + 更新浏览数量失败: {}",
                    e
                ))
            }
        }
    }
}

//////// END
