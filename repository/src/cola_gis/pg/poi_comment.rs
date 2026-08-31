// repository/src/cola_gis/pg/add  -- 仓储 - GIS - pg - 兴趣点 评论
// 2026/7/6 14:20

////////

use crate::pg_pool;
use cola_data::cola_gis::command::comment::PoiCommentCommand;
use cola_data::cola_gis::entity::comment::{POI_COMMENT_COLUMNS, PoiCommentEntity};
use sqlx;

////////

/// # [COMMENT REPOSITRY] - 兴趣点 评论
pub struct PoiCommentRepo;

// 构造实现
impl PoiCommentRepo {
    // 💡

    ////////

    /// # 1. [REPOSITORY] - 最新的评论列表
    pub async fn find_new_comments_by_poi_id(
        poi_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PoiCommentEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.poi_comments WHERE status = 1 AND poi_id = $1 ORDER BY add_time DESC LIMIT $2 OFFSET $3",
            POI_COMMENT_COLUMNS
        );
        sqlx::query_as::<_, PoiCommentEntity>(&query)
            .bind(poi_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 热门的评论列表
    pub async fn find_hot_comments_by_poi_id(
        poi_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PoiCommentEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.poi_comments WHERE status = 1 AND poi_id = $1 ORDER BY likes DESC, add_time DESC LIMIT $2 OFFSET $3",
            POI_COMMENT_COLUMNS
        );
        sqlx::query_as::<_, PoiCommentEntity>(&query)
            .bind(poi_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    pub async fn find_comments_by_user_id(
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<PoiCommentEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_gis.poi_comments WHERE status = 1 AND uid = $1 ORDER BY add_time DESC LIMIT $2 OFFSET $3",
            POI_COMMENT_COLUMNS
        );
        sqlx::query_as::<_, PoiCommentEntity>(&query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 保存新评论
    pub async fn save_comment_by_poi_id(
        user_id: i64,
        poi_id: i64,
        cmd: PoiCommentCommand,
        visibility: i16,
    ) -> Result<PoiCommentEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "INSERT INTO cola_gis.poi_comment (user_id, poi_id, parent_id, content, visibility, status) \
             VALUES ($1, $2, $3, $4, $5, 1) RETURNING {}",
            POI_COMMENT_COLUMNS
        );
        sqlx::query_as::<_, PoiCommentEntity>(&query)
            .bind(user_id)
            .bind(poi_id)
            .bind(cmd.parent_id)
            .bind(cmd.content)
            .bind(visibility)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 用户删除评论
    pub async fn user_del_comment_by_id(
        uid: i64,
        comment_id: i64,
    ) -> Result<PoiCommentEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "UPDATE cola_gis.poi_comment SET status = 0, updated_at = NOW() WHERE id = $1 AND uid = $2 RETURNING {}",
            POI_COMMENT_COLUMNS
        );
        sqlx::query_as::<_, PoiCommentEntity>(&query)
            .bind(comment_id)
            .bind(uid)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 5. [REPOSITORY] - 同步删除评论
    pub async fn sync_soft_del_comments_by_poi_id(poi_id: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let query = r#"UPDATE cola_gis.poi_comment SET status = 0, deleted_at = NOW(), deleted_by = -1 WHERE poi_id = $1 AND status = 1"#;
        let result = sqlx::query(query).bind(poi_id).execute(&pool).await?;
        Ok(result.rows_affected())
    }
}

//////// END
