// repository/src/gis/pg/permission.rs
//  仓储 - GIS - USER - 权限
// 2026/8/11 11:02 Created.

////////
pub mod check;

////////

/// # [PERMISSION REPOSITORY] - 权限仓储
/// * `desc`: `修改用户权限`
pub struct GisPermissionRepo;

impl GisPermissionRepo {
    /// # 1. [REPOSITORY] - 更新用户权限
    pub async fn update_user_permission(uid: i64, delta: i32) -> Result<(), sqlx::Error> {
        let pool = crate::pg_pool();
        let sql = r#"UPDATE cola_gis.gis_user SET publish_count = GREATEST(0, publish_count + $2), updated_at = NOW() WHERE uid = $1"#;
        sqlx::query(sql)
            .bind(uid)
            .bind(delta)
            .execute(&pool)
            .await?;
        Ok(())
    }

    /// # 2. [REPOSITORY] - 更新评论权限
    pub async fn update_video_comment_perm(
        uid: i64,
        video_id: i64,
        comment_perm: i16,
    ) -> Result<(), sqlx::Error> {
        let pool = crate::pg_pool();
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
