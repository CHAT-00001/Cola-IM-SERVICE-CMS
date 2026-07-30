// repo_adapter/src/video/follow
// 2026-06-12 09:30

////////

use async_trait::async_trait;
use cola_data::video::port::like::LikeRepo;
use repository::video::service::like::LikeService;

////////


/// # [LIKE PORT] - 点赞 端口 插头
pub struct LikePortAdapter;

////////

#[async_trait]
impl LikeRepo for LikePortAdapter {

    ////////

    /// # 1. [PORT] - 保存点赞记录 + 更新点赞数量
    async fn like_video(
        &self,
        uid: i64,
        video_id: i64,
        is_liked: bool,
    ) -> anyhow::Result<()> {
        LikeService::save_like_with_update_video_count(uid, video_id, is_liked)
            .await
            .map_err(|e| anyhow::anyhow!("like_video failed: {}", e))?;
        Ok(())
    }

    ////////

    /// # 2. [PORT] - 保存不喜欢记录 + 更新不喜欢数量
    async fn unlike_video(
        &self,
        uid: i64,
        video_id: i64,
        is_unliked: bool,
    ) -> anyhow::Result<()> {
        LikeService::save_unlike_with_update_video_count(uid, video_id, is_unliked)
            .await
            .map_err(|e| anyhow::anyhow!("unlike_video failed: {}", e))?;
        Ok(())
    }
}

//////// END