// service/src/cola_video/user/permission_change.rs
// 👤 服务 - 可乐视频 - 创作者 - 权限 - 服务
// 2026/7/6 15:53 Created.

////////

use anyhow::Result;
use repository::pg_pool;

////////

/// # [CHANGE SERVICE] - 修改
/// * `desc`: `可乐GIS - 修改权限`
pub struct PermissionsChangeService;

impl PermissionsChangeService {
    pub async fn update_user_permission(uid: i64, delta: i32) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"UPDATE cola_gis.gis_user SET publish_count = GREATEST(0, publish_count + $2), updated_at = NOW() WHERE uid = $1"#;
        sqlx::query(sql)
            .bind(uid)
            .bind(delta)
            .execute(&pool)
            .await?;
        Ok(())
    }

    pub async fn check_video_permission(user_id: i64) -> Result<i16, sqlx::Error> {
        Ok(1)
    }

    pub async fn update_video_comment_perm(
        uid: i64,
        video_id: i64,
        comment_perm: i16,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"UPDATE cola_gis.cola_gis SET comment_perm = $3, updated_at = NOW() WHERE id = $2 AND uid = $1"#;
        sqlx::query(sql)
            .bind(uid)
            .bind(video_id)
            .bind(comment_perm)
            .execute(&pool)
            .await?;
        Ok(())
    }
}

//////// END
