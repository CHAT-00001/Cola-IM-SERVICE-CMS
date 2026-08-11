// service/src/video/permission/check.rs
// 服务 - VIDEO - 权限 - 检查
// 2026/8/11 11:19 Created.

////////

use anyhow::Result;
use repository::cola_video::pg::permission::check::VideoPermissionCheckRepo;

////////

/// # [CHECK SERVICE] - 检查
/// * `desc`: `视频权限检查服务`
pub struct VideoPermissionsCheckService;

// 构造实现
impl VideoPermissionsCheckService {
    //

    ////////

    /// # 1. [SERVICE] - 发布权限
    pub async fn check_video_publish_perm(uid: i64) -> Result<i16> {
        let perm = VideoPermissionCheckRepo::get_publish_perm(uid)
            .await
            .map_err(|e| anyhow::anyhow!("检查视频发布权限失败: {}", e))?;

        Ok(perm)
    }

    ////////

    /// # 2. [SERVICE] - 可见权限
    pub async fn check_video_visibility_perm(uid: i64, delta: i32) -> Result<()> {
        let _ = delta; // 预留参数

        // 调用底层 Repository
        let _perm = VideoPermissionCheckRepo::get_visibility_perm(uid)
            .await
            .map_err(|e| anyhow::anyhow!("检查视频可见性权限失败: {}", e))?;

        Ok(())
    }
}

//////// END