// cola_video/src/live/biz/interact.rs  -- 互动逻辑
// 2026-03-27 06:22

use anyhow::{Result, anyhow};
use tracing::{info, warn};

////////

use crate::video::port::like::LikePort;

////////
pub struct VideoCommentLogic;

impl VideoCommentLogic {
    /// ## LOGIC - 添加点赞逻辑
    pub async fn add_like_logic(
        uid: i64,
        video_id: i64,
    ) -> Result<()> {
        // 1. 检查记录是否存在 (通过 Port 调用)
        let is_exist = interact_port
            .check_like_exists(uid, video_id)
            .await
            .map_err(|e| anyhow!("检查点赞记录失败: {}", e))?;

        if is_exist.is_some() {
            return Err(anyhow!("你已经点过赞了"));
        }

        // 2. 执行持久化
        // 自动 Repo 模式：适配器内部应实现事务或缓存同步
        interact_port
            .save_like_and_incr_count(uid, video_id)
            .await
            .map_err(|e| anyhow!("保存点赞记录失败: {}", e))?;

        info!("用户 {} 点赞了视频 {}", uid, video_id);
        Ok(())
    }

    /// ## 移除点赞逻辑
    pub async fn remove_like_logic(
        uid: i64,
        video_id: i64,
        interact_port: &dyn LikePort,
    ) -> Result<()> {
        // 检查是否存在
        let is_exist = interact_port
            .check_like_exists(uid, video_id)
            .await
            .map_err(|e| anyhow!("检查点赞记录失败: {}", e))?;
        if is_exist.is_none() {
            return Err(anyhow!("点赞记录不存在，无法取消"));
        }

        // 移除记录并减少计数
        interact_port
            .delete_like_and_decr_count(uid, video_id)
            .await
            .map_err(|e| anyhow!("取消点赞失败: {}", e))?;

        Ok(())
    }

    /// ## 收藏视频逻辑
    pub async fn add_collect_logic(
        uid: i64,
        video_id: i64,
        interact_port: &dyn LikePort,
    ) -> Result<()> {
        // 1. 获取当前收藏数 (自动从存储层获取)
        let count = interact_port
            .get_user_collect_count(uid)
            .await
            .map_err(|e| anyhow!("获取收藏数失败: {}", e))?;

        // 2. 校验
        if count >= 500 {
            return Err(anyhow!("收藏文件夹已满（上限500个）"));
        }

        // 3. 执行收藏
        interact_port
            .save_collect_record(uid, video_id)
            .await
            .map_err(|e| anyhow!("保存收藏记录失败: {}", e))?;

        Ok(())
    }

    /// ## 移除收藏逻辑
    pub async fn remove_collect_logic(
        uid: i64,
        video_id: i64,
        interact_port: &dyn LikePort,
    ) -> Result<()> {
        interact_port
            .delete_collect_record(uid, video_id)
            .await
            .map_err(|e| anyhow!("删除收藏记录失败: {}", e))?;
        Ok(())
    }

    /// ## 增加浏览量逻辑
    pub async fn increment_view_logic(video_id: i64, interact_port: &dyn LikePort) -> Result<()> {
        // 适配器内部可以实现异步写入或 Redis 缓冲
        interact_port
            .update_view_count(video_id)
            .await
            .map_err(|e| anyhow!("更新浏览量失败: {}", e))?;
        Ok(())
    }

    /// ## 报告完播逻辑
    pub async fn mark_done_logic(
        uid: i64,
        video_id: i64,
        interact_port: &dyn LikePort,
    ) -> Result<()> {
        interact_port
            .save_play_done_record(uid, video_id)
            .await
            .map_err(|e| anyhow!("保存播放完成记录失败: {}", e))?;
        Ok(())
    }

    /// ## 举报视频逻辑
    pub async fn report_logic(
        uid: i64,
        video_id: i64,
        reason_type: i16,
        interact_port: &dyn LikePort,
    ) -> Result<()> {
        if reason_type <= 0 {
            return Err(anyhow!("举报理由无效"));
        }

        interact_port
            .save_report_record(uid, video_id, reason_type)
            .await
            .map_err(|e| anyhow!("保存举报记录失败: {}", e))?;
        Ok(())
    }

    /// ## 设置热门逻辑 (管理端)
    pub async fn set_hot_logic(video_id: i64, interact_port: &dyn LikePort) -> Result<()> {
        interact_port
            .update_video_hot_status(video_id, true)
            .await
            .map_err(|e| anyhow!("更新视频热门状态失败: {}", e))?;
        Ok(())
    }
}

//////// END
