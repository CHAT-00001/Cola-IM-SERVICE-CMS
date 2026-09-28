// repo_adapter/src/cola_video/cola_video/check.rs -- ADAPTER - VIDEO - CONTENT - 检查适配器
// 2026/8/6 19:19 Created.

////////

use anyhow::Result;
use async_trait::async_trait;
use port::cola_video::video::check::VideoCheckPort;
use port::cola_video::video::check::VideoCommentAccess;
use repository::video::pg::video::check::VideoCheckRepo;
use repository::video::redis::like::VideoCommentAccessRedisRepo;

////////

/// # [CHECK ADAPTER] - 视频内容检查适配器
/// * `desc`: `VIDEO - Content Check Adapter.`
pub struct VideoCheckAdapter;

#[async_trait]
impl VideoCheckPort for VideoCheckAdapter {
    async fn check_health(&self, video_id: i64) -> Result<(bool)> {
        todo!()
    }

    async fn check_state(&self, video_id: i64) -> Result<(bool)> {
        todo!()
    }

    async fn is_owner(&self, uid: i64, video_id: i64) -> Result<(bool)> {
        todo!()
    }

    ////////

    /// # 4. [ADAPTER] - 检查视频评论访问状态
    /// * `desc`: `根据视频状态、删除状态和评论权限返回真实结果`
    async fn check_comment_access(&self, video_id: i64) -> Result<VideoCommentAccess> {
        if let Some(access) = VideoCommentAccessRedisRepo::get(video_id).await? {
            return Ok(match access.as_str() {
                "allowed" => VideoCommentAccess::Allowed,
                "forbidden" => VideoCommentAccess::Forbidden,
                _ => VideoCommentAccess::NotFound,
            });
        }

        let access = match VideoCheckRepo::find_comment_access(video_id).await? {
            None => VideoCommentAccess::NotFound,
            Some((status, comment_perm, is_del, is_deleted))
                if status != 1 || is_del != 0 || is_deleted.unwrap_or(false) =>
            {
                VideoCommentAccess::NotFound
            }
            Some((_, comment_perm, _, _)) if comment_perm <= 0 => VideoCommentAccess::Forbidden,
            Some(_) => VideoCommentAccess::Allowed,
        };

        let cache_value = match access {
            VideoCommentAccess::Allowed => "allowed",
            VideoCommentAccess::Forbidden => "forbidden",
            VideoCommentAccess::NotFound => "not_found",
        };
        if let Err(error) = VideoCommentAccessRedisRepo::set(video_id, cache_value).await {
            tracing::warn!(
                "[🤐 ADAPTER] - ❌️ 视频评论权限缓存写入失败: video_id={}, error={}",
                video_id,
                error
            );
        }
        Ok(access)
    }
}

//////// END
