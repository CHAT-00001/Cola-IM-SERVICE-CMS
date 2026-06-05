// cola_video/src/video/biz/change.rs  -- 核心 - 短视频 - 业务 - 修改
// 2026/5/20 03:19 by wx: cestbon10080

////////

use anyhow::Result;
use cola_data::video::command::video::VideoCommand;
use tracing::{info, warn};

////////

pub struct VideoChangeLogic;

impl VideoChangeLogic {
    /// # 1. [LOGIC] - 编辑视频
    pub async fn logic_change_edit(
        uid: i64,
        cmd: VideoCommand,
        change_port: &dyn ChangePort,
    ) -> Result<bool> {
        change_port
            .update_video(uid, cmd)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 视频编辑持久化失败: {}", e))?;

        info!("BIZ - 视频编辑成功: uid={}", uid);
        Ok(true)
    }

    ////////

    /// # 2. [LOGIC] - 浏览权限
    pub async fn logic_change_visibility_range(
        uid: i64,
        video_id: i64,
        range: i16,
        change_port: &dyn ChangePort,
    ) -> Result<bool> {
        change_port
            .update_visibility_permission(uid, video_id, range)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 浏览权限同步失败: {}", e))?;

        info!("BIZ - 浏览权限修改成功: uid={}, video_id={}", uid, video_id);
        Ok(true)
    }

    ////////

    /// # 3. [LOGIC] - 评论权限
    pub async fn logic_change_comment_range(
        uid: i64,
        video_id: i64,
        range: i16,
        change_port: &dyn ChangePort,
    ) -> Result<bool> {
        change_port
            .update_comment_permission(uid, video_id, range)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 评论权限同步失败: {}", e))?;

        info!("BIZ - 评论权限修改成功: uid={}, video_id={}", uid, video_id);
        Ok(true)
    }

    ////////

    /// # 4. [LOGIC] - 弹幕权限
    pub async fn logic_change_danmaku_range(
        uid: i64,
        video_id: i64,
        range: i16,
        change_port: &dyn ChangePort,
    ) -> Result<bool> {
        change_port
            .update_danmaku_permission(uid, video_id, range)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 弹幕权限同步失败: {}", e))?;

        info!("BIZ - 弹幕权限修改成功: uid={}, video_id={}", uid, video_id);
        Ok(true)
    }

    //////

    /// # 5. [LOGIC] - 收藏权限
    pub async fn logic_change_collect_range(
        uid: i64,
        video_id: i64,
        range: i16,
        change_port: &dyn ChangePort,
    ) -> Result<bool> {
        change_port
            .update_collect_permission(uid, video_id, range)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 收藏权限同步失败: {}", e))?;

        info!("BIZ - 收藏权限修改成功: uid={}, video_id={}", uid, video_id);
        Ok(true)
    }

    ////////

    /// # 6. [LOGIC] - 下载权限
    pub async fn logic_change_download_range(
        uid: i64,
        video_id: i64,
        range: i16,
        change_port: &dyn ChangePort,
    ) -> Result<bool> {
        change_port
            .update_download_permission(uid, video_id, range)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 下载权限同步失败: {}", e))?;

        info!("BIZ - 下载权限修改成功: uid={}, video_id={}", uid, video_id);
        Ok(true)
    }

    ////////

    /// # 7. [LOGIC] - 购买权限
    pub async fn logic_change_buy_range(
        uid: i64,
        video_id: i64,
        range: i16,
        change_port: &dyn ChangePort,
    ) -> Result<bool> {
        change_port
            .update_buy_permission(uid, video_id, range)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 购买权限同步失败: {}", e))?;

        info!("BIZ - 购买权限修改成功: uid={}, video_id={}", uid, video_id);
        Ok(true)
    }

    ////////

    /// # 8. [LOGIC] - 更新播放进度
    pub async fn logic_update_play_progress(
        uid: i64,
        video_id: i64,
        play_count: Option<i16>,
        progress: i32,
        change_port: &dyn ChangePort,
    ) -> Result<bool> {
        let is_finished = progress > 0;
        change_port
            .update_play_progress(uid, video_id, play_count, progress, is_finished)
            .await
            .map_err(|e| anyhow::anyhow!("BIZ: 播放进度更新失败: {}", e))?;

        info!("BIZ - 播放进度更新成功: uid={}, video_id={}", uid, video_id);
        Ok(true)
    }
}

//////// END
