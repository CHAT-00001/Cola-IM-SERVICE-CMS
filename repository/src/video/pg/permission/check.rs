// repository/src/cola_video/pg/permission/check.rs
// 仓储 - VIDEO - 权限 - 检查
// 2026/8/11 11:12 Created.

////////

use crate::pg_pool;

////////

/// # [CHECK REPOSITORY] - 权限检查仓储
pub struct VideoPermissionCheckRepo;

impl VideoPermissionCheckRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 获取视频发布权限
    pub async fn get_publish_perm(uid: i64) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"SELECT publish_perm FROM cola_video.video_user WHERE uid = $1"#;
        let perm = sqlx::query_scalar::<_, i16>(sql)
            .bind(uid)
            .fetch_one(&pool)
            .await?;
        Ok(perm)
    }

    ////////

    /// # 2. [REPOSITORY] - 获取视频可见性权限
    pub async fn get_visibility_perm(id: i64) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"SELECT visibility_perm FROM cola_gis.cola_gis WHERE id = $1"#;
        let perm = sqlx::query_scalar::<_, i16>(sql)
            .bind(id)
            .fetch_one(&pool)
            .await?;
        Ok(perm)
    }
}

//////// END
