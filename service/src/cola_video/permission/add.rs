// service/src/video/permission/add.rs
// 服务 - VIDEO - 创作者权限 - 修改
// 2026/8/11 11:19 Created.

////////

use anyhow::Result;
use repository::cola_video::pg::permission::add::VideoPermissionAddRepo;

////////

/// # [ADD SERVICE] - 修改
/// * `desc`: `可乐视频 - 修改权限`
pub struct PermissionsChangeService;

impl PermissionsChangeService {
    pub async fn update_user_permission(uid: i64, delta: i32) -> Result<()> {
        VideoPermissionAddRepo::update_user_permission(uid, delta)
            .await
            .map_err(|e| anyhow::anyhow!("更新用户权限失败: {}", e))?;
        Ok(())
    }

    pub async fn check_video_permission(user_id: i64) -> Result<i16> {
        let _ = user_id;
        Ok(1)
    }

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
}

//////// END