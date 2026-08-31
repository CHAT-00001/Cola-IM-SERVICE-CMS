// service/src/cola_gis/permission_change.rs
// 服务 - 可乐GIS - 权限 - 服务
// 2026/7/6

////////

use anyhow::Result;
use repository::cola_gis::pg::permission::GisPermissionRepo;

////////

/// # [CHANGE SERVICE] - 修改
/// * `desc`: `可乐GIS - 修改权限`
pub struct PermissionsChangeService;

impl PermissionsChangeService {
    pub async fn update_user_permission(uid: i64, delta: i32) -> Result<()> {
        GisPermissionRepo::update_user_permission(uid, delta)
            .await
            .map_err(|e| anyhow::anyhow!("更新用户权限失败: {}", e))?;
        Ok(())
    }

    pub async fn check_video_permission(user_id: i64) -> Result<i16> {
        let _ = user_id; // 避免未使用参数警告
        Ok(1)
    }

    pub async fn update_video_comment_perm(
        uid: i64,
        video_id: i64,
        comment_perm: i16,
    ) -> Result<()> {
        GisPermissionRepo::update_video_comment_perm(uid, video_id, comment_perm)
            .await
            .map_err(|e| anyhow::anyhow!("更新视频评论权限失败: {}", e))?;
        Ok(())
    }
}

//////// END
