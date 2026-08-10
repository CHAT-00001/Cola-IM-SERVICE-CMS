// cola_user/src/case/cola_user/get.rs
// core - USER - case - cola_user - 获取 用例
// 2026-03-30 08:25

////////

use anyhow::{Result, anyhow};
use cola_data::cola_video::info::video::VideoInfo;
use port::ctx::AppContext;
use tracing::info;

////////

/// # [GET CASE] - 浏览 用例
pub struct UserGetCase;

impl UserGetCase {
    //

    ////////

    /// # 1. [CASE] - 保存主页浏览记录
    /// * `desc`: 记录一次浏览 + 更新主页浏览量
    pub async fn case_add_video_view(
        uid: i64,         // 操作者ID
        video_id: i64,    // 主页 ID
        ctx: &AppContext, // 全局上下文
    ) -> Result<(), anyhow::Error> {
        ctx.video
            .view
            .add
            .save_view(uid, video_id)
            .await
            .map_err(|e| anyhow!("[🤐 USER GET CASE]: ❌️ 保存浏览记录失败: {}", e))?;

        info!(
            "[🗣️ USER GET CASE]: ✅️ 保存浏览记录成功, uid={}, video_id={}",
            uid, video_id
        );
        Ok(())
    }

    ////////

    /// # 2. [CASE] - 获取视频详情
    /// * `desc`: 根据视频ID查询视频Info
    pub async fn case_get_video_detail(
        _uid: i64,        // 操作者ID
        video_id: i64,    // 视频ID
        ctx: &AppContext, // 全局上下文
    ) -> Result<Option<VideoInfo>, anyhow::Error> {
        let infos = ctx
            .video
            .view
            .get
            .get_video_list_by_ids(vec![video_id])
            .await
            .map_err(|e| anyhow!("[🤐 USER GET CASE]: ❌️ 查询视频详情失败: {}", e))?;

        if let Some(info) = infos.into_iter().next() {
            info!(
                "[🗣️ USER GET CASE]: ✅️ 查询视频详情成功, video_id={}",
                video_id
            );
            Ok(Some(info))
        } else {
            info!("[🗣️ USER GET CASE]: ✅️ 视频不存在, video_id={}", video_id);
            Ok(None)
        }
    }
}

//////// END
