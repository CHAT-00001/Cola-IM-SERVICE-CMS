// repository/src/video/pg/video/check.rs -- 仓储 - VIDEO - PG - 视频 - 检查仓储
// 2026/8/11 11:27 Created.

////////

use crate::pg_pool;

////////

/// # [CHECK REPOSITORY] - 检查
/// * `desc`: `视频权限检查仓储`
pub struct VideoCheckRepo;

impl VideoCheckRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 检查视频发布权限
    pub async fn check_video_publish_perm(uid: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"
            SELECT publish_perm
            FROM cola_video.video
            WHERE uid = $1
        "#;
        sqlx::query(sql).bind(uid).execute(&pool).await?;
        Ok(())
    }

    ////////

    /// # 4. [REPOSITORY] - 查询视频评论访问状态
    /// * `desc`: `返回 status、删除状态和评论权限原始值`
    pub async fn find_comment_access(
        video_id: i64,
    ) -> Result<Option<(i16, i16, i16, Option<bool>)>, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_as(
            r#"
            SELECT status, comment_perm, is_del, is_deleted
            FROM cola_video.video
            WHERE id = $1
            LIMIT 1
            "#,
        )
        .bind(video_id)
        .fetch_optional(&pool)
        .await
    }

    /// # 2. [REPOSITORY] - 检查视频编辑权限
    pub async fn check_video_edit_perm(uid: i64, delta: i32) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"
            UPDATE cola_video.video
            SET views = COALESCE(views, 0) + $2
            WHERE id = $1
        "#;
        sqlx::query(sql)
            .bind(uid)
            .bind(delta)
            .execute(&pool)
            .await?;
        Ok(())
    }

    ////////

    /// # 3. [REPOSITORY] - 检查视频浏览权限
    pub async fn check_video_visibility_perm(uid: i64, delta: i32) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"
            UPDATE cola_video.video
            SET views = COALESCE(views, 0) + $2
            WHERE id = $1
        "#;
        sqlx::query(sql)
            .bind(uid)
            .bind(delta)
            .execute(&pool)
            .await?;
        Ok(())
    }

    ////////

    /// # 4. [REPOSITORY] - 检查视频评论权限
    pub async fn check_video_comment_perm(user_id: i64) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"
            SELECT perm_id
            FROM cola_video.video
            WHERE id = $1
            LIMIT 1
        "#;
        let perm_id: (i16,) = sqlx::query_as(sql).bind(user_id).fetch_one(&pool).await?;
        Ok(perm_id.0)
    }

    ////////

    /// # 5. [REPOSITORY] - 检查视频弹幕权限
    pub async fn check_video_danmaku_perm(uid: i64, new_perm_id: i16) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"
            UPDATE cola_video.video
            SET perm_id = $2, updated_at = NOW()
            WHERE id = $1
        "#;
        sqlx::query(sql)
            .bind(uid)
            .bind(new_perm_id)
            .execute(&pool)
            .await?;
        Ok(())
    }

    ////////

    /// # 6. [REPOSITORY] - 检查视频收藏权限
    pub async fn check_video_collect_perm(uid: i64, new_perm_id: i16) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"
            UPDATE cola_video.video
            SET perm_id = $2, updated_at = NOW()
            WHERE id = $1
        "#;
        sqlx::query(sql)
            .bind(uid)
            .bind(new_perm_id)
            .execute(&pool)
            .await?;
        Ok(())
    }

    ////////

    /// # 7. [REPOSITORY] - 检查视频下载权限
    pub async fn check_video_download_perm(
        uid: i64,
        video_id: i64,
        comment_perm: i16,
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"
            UPDATE cola_video.video
            SET comment_perm = $3, updated_at = NOW()
            WHERE id = $2 AND user_id = $1
        "#;
        let result = sqlx::query(sql)
            .bind(uid)
            .bind(video_id)
            .bind(comment_perm)
            .execute(&pool)
            .await?;
        Ok(result.rows_affected())
    }

    ////////

    /// # 8. [REPOSITORY] - 检查视频购买权限
    pub async fn check_video_buy_perm(
        uid: i64,
        video_id: i64,
        danmaku_perm: i16,
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"
            UPDATE cola_video.video
            SET danmaku_perm = $3, updated_at = NOW()
            WHERE id = $2
        "#;
        let result = sqlx::query(sql)
            .bind(uid)
            .bind(video_id)
            .bind(danmaku_perm)
            .execute(&pool)
            .await?;
        Ok(result.rows_affected())
    }
}

//////// END
