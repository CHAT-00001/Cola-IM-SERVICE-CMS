// service/src/cola_gis/permission/check.rs
// 服务 - GIS - 权限 - 模块
// 2026/7/6

////////

use anyhow::Result;
// 引入对应的 Repository（请根据实际模块路径调整）
use repository::cola_gis::pg::permission::check::GisPermissionCheckRepo;

////////

/// # [PERMISSIONS CHECK SERVICE] - 权限检查服务
pub struct VideoPermissionsCheckService;

// 构造实现
impl VideoPermissionsCheckService {
    pub async fn check_video_publish_perm(uid: i64) -> Result<()> {
        let _ = uid;
        Ok(())
    }

    pub async fn check_video_visibility_perm(uid: i64, delta: i32) -> Result<()> {
        let _ = delta; // 如果 delta 暂时不用，保留参数定义防止接口断裂

        // 调用底层 Repository 查询数据
        let _perm = GisPermissionCheckRepo::get_visibility_perm(uid)
            .await
            .map_err(|e| anyhow::anyhow!("检查视频可见性权限失败: {}", e))?;

        Ok(())
    }
}

//////// END