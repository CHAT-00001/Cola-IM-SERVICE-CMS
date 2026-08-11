// service/src/cola_video/video/check.rs
// 👤 服务 - 可乐视频 - 视频 - 检查
// 2026/6/8 23:33

////////

use anyhow::Result;
use repository::cola_video::pg::video::check::VideoCheckRepo;
use tracing::error;

////////

/// # [SERVICE] - 短视频权限服务
pub struct VideoPermissionsCheckService;

// 构造函数
impl VideoPermissionsCheckService {
    //

    ////////

    /// # 1. [SERVICE] - 检查视频发布权限
    pub async fn check_video_publish_perm(uid: i64) -> Result<()> {
        VideoCheckRepo::check_video_publish_perm(uid)
            .await
            .map_err(|err| {
                error!(
                    uid = uid,
                    error = ?err,
                    "check_video_publish_perm failed"
                );
                err
            })?;

        Ok(())
    }

    ////////

    /// # 2. [SERVICE] - 检查视频编辑权限
    pub async fn check_video_edit_perm(uid: i64, delta: i32) -> Result<()> {
        VideoCheckRepo::check_video_edit_perm(uid, delta)
            .await
            .map_err(|e| anyhow::anyhow!("检查视频编辑权限失败: {}", e))?;
        Ok(())
    }

    ////////

    /// # 3. [SERVICE] - 检查视频浏览权限
    pub async fn check_video_visibility_perm(uid: i64, delta: i32) -> Result<()> {
        VideoCheckRepo::check_video_visibility_perm(uid, delta)
            .await
            .map_err(|e| anyhow::anyhow!("检查视频可见性权限失败: {}", e))?;
        Ok(())
    }

    ////////

    /// # 4. [SERVICE] - 检查视频评论权限
    pub async fn check_video_comment_perm(user_id: i64) -> Result<i16> {
        let perm_id = VideoCheckRepo::check_video_comment_perm(user_id)
            .await
            .map_err(|e| anyhow::anyhow!("检查视频评论权限失败: {}", e))?;
        Ok(perm_id)
    }

    ////////

    /// # 5. [SERVICE] - 检查视频弹幕权限
    pub async fn check_video_danmaku_perm(uid: i64, new_perm_id: i16) -> Result<()> {
        VideoCheckRepo::check_video_danmaku_perm(uid, new_perm_id)
            .await
            .map_err(|e| anyhow::anyhow!("检查视频弹幕权限失败: {}", e))?;
        Ok(())
    }

    ////////

    /// # 6. [SERVICE] - 检查视频收藏权限
    pub async fn check_video_collect_perm(uid: i64, new_perm_id: i16) -> Result<()> {
        VideoCheckRepo::check_video_collect_perm(uid, new_perm_id)
            .await
            .map_err(|e| anyhow::anyhow!("检查视频收藏权限失败: {}", e))?;
        Ok(())
    }

    ////////

    /// # 7. [SERVICE] - 检查视频下载权限
    pub async fn check_video_download_perm(
        uid: i64,
        video_id: i64,
        comment_perm: i16,
    ) -> Result<()> {
        let rows_affected = VideoCheckRepo::check_video_download_perm(uid, video_id, comment_perm)
            .await
            .map_err(|e| anyhow::anyhow!("检查视频下载权限失败: {}", e))?;

        if rows_affected == 0 {
            return Err(anyhow::anyhow!("RowNotFound"));
        }

        Ok(())
    }

    ////////

    /// # 8. [SERVICE] - 检查视频购买权限
    pub async fn check_video_buy_perm(uid: i64, video_id: i64, danmaku_perm: i16) -> Result<()> {
        let rows_affected = VideoCheckRepo::check_video_buy_perm(uid, video_id, danmaku_perm)
            .await
            .map_err(|e| anyhow::anyhow!("检查视频购买权限失败: {}", e))?;

        if rows_affected == 0 {
            return Err(anyhow::anyhow!("RowNotFound"));
        }

        Ok(())
    }
}

//////// END
