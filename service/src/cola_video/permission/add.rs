// service/src/video/permission/add.rs
// 服务 - VIDEO - 创作者权限 - 修改
// 2026/8/11 11:19 Created.

////////

use anyhow::Result;
use repository::cola_video::pg::permission::add::VideoPermissionAddRepo;

////////

/// # [ADD SERVICE] - 修改
/// * `desc`: `可乐视频 - 用户权限修改`
pub struct PermissionsChangeService;

impl PermissionsChangeService {
    //

    ////////

    /// # [SERVICE] - 更新用户权限
    pub async fn update_user_permission(uid: i64, delta: i32) -> Result<()> {
        VideoPermissionAddRepo::update_user_permission(uid, delta)
            .await
            .map_err(|e| anyhow::anyhow!("更新用户权限失败: {}", e))?;
        Ok(())
    }

    ////////

    /// # [SERVICE] - 更新浏览权限
    pub async fn check_video_permission(user_id: i64) -> Result<i16> {
        let _ = user_id;
        Ok(1)
    }

    ////////

    /// # [SERVICE] - 更新评论权限
    pub async fn update_video_comment_perm(
        uid: i64,
        video_id: i64,
        comment_perm: i16,
    ) -> Result<()> {
        VideoPermissionAddRepo::update_video_comment_perm(uid, video_id, comment_perm)
            .await
            .map_err(|e| anyhow::anyhow!("更新视频评论权限失败: {}", e))?;
        Ok(())
    }

    ////////

    /// # [SERVICE] - 更新弹幕权限
    pub async fn update_video_danmaku_perm(
        uid: i64,
        video_id: i64,
        comment_perm: i16,
    ) -> Result<()> {
        VideoPermissionAddRepo::update_video_comment_perm(uid, video_id, comment_perm)
            .await
            .map_err(|e| anyhow::anyhow!("更新弹幕评论权限失败: {}", e))?;
        Ok(())
    }

    ////////

    /// # [SERVICE] - 更新收藏权限
    pub async fn update_video_collect_perm(
        uid: i64,
        video_id: i64,
        comment_perm: i16,
    ) -> Result<()> {
        VideoPermissionAddRepo::update_video_comment_perm(uid, video_id, comment_perm)
            .await
            .map_err(|e| anyhow::anyhow!("更新弹幕评论权限失败: {}", e))?;
        Ok(())
    }

    ////////

    /// # [SERVICE] - 更新下载权限
    pub async fn update_video_download_perm(
        uid: i64,
        video_id: i64,
        comment_perm: i16,
    ) -> Result<()> {
        VideoPermissionAddRepo::update_video_comment_perm(uid, video_id, comment_perm)
            .await
            .map_err(|e| anyhow::anyhow!("更新弹幕评论权限失败: {}", e))?;
        Ok(())
    }

    ////////

    /// # [SERVICE] - 更新购买权限
    pub async fn update_video_buy_perm(uid: i64, video_id: i64, comment_perm: i16) -> Result<()> {
        VideoPermissionAddRepo::update_video_comment_perm(uid, video_id, comment_perm)
            .await
            .map_err(|e| anyhow::anyhow!("更新弹幕评论权限失败: {}", e))?;
        Ok(())
    }
}

//////// END
