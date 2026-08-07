// repository/src/new/pg/new/perm_check.rs  --  权限检查 仓储
// 仓储 - VIDEO - pg - new - 权限检查
// 2026/6/9 11:20

////////
use anyhow::Result;
use crate::pg_pool;
use sqlx::Row;

/// # [REPOSITORY] - 视频权限检查仓储
pub struct VideoPermCheckRepo;

impl VideoPermCheckRepo {

    /// # 1. 检查视频浏览权限
    pub async fn check_video_visibility_perm(video_id: i64) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"SELECT visibility_perm FROM "new" WHERE id = $1 LIMIT 1"#;

        // 使用 query_scalar 直接获取第一个字段的值
        sqlx::query_scalar(sql).bind(video_id).fetch_one(&pool).await
    }

    /// # 2. 检查视频评论权限
    pub async fn check_video_comment_perm(video_id: i64) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"SELECT comment_perm FROM "new" WHERE id = $1 LIMIT 1"#;

        sqlx::query_scalar(sql).bind(video_id).fetch_one(&pool).await
    }

    /// # 3. 检查视频弹幕权限
    pub async fn check_video_danmaku_perm(video_id: i64) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"SELECT danmaku_perm FROM "new" WHERE id = $1 LIMIT 1"#;

        sqlx::query_scalar(sql).bind(video_id).fetch_one(&pool).await
    }

    /// # 4. 检查视频收藏权限
    pub async fn check_video_collect_perm(video_id: i64) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"SELECT collect_perm FROM "new" WHERE id = $1 LIMIT 1"#;

        sqlx::query_scalar(sql).bind(video_id).fetch_one(&pool).await
    }

    /// # 5. 检查视频下载权限
    pub async fn check_video_download_perm(video_id: i64) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"SELECT download_perm FROM "new" WHERE id = $1 LIMIT 1"#;

        sqlx::query_scalar(sql).bind(video_id).fetch_one(&pool).await
    }

    /// # 6. 检查视频购买权限
    pub async fn check_video_buy_perm(video_id: i64) -> Result<i16, sqlx::Error> {
        let pool = pg_pool();
        let sql = r#"SELECT buy_perm FROM "new" WHERE id = $1 LIMIT 1"#;

        sqlx::query_scalar(sql).bind(video_id).fetch_one(&pool).await
    }
}


//////// END