// repository/src/cola_fs/pg/media.rs  -- 仓储 - FS - PG - 媒体资源仓储
// 2026/8/14 13:30

////////

use chrono::{DateTime, Utc};
use cola_data::cola_fs::command::media::CreateMediaCmd;
use cola_data::cola_fs::entity::media::{MEDIA_COLUMNS, MediaEntity};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

////////

/// # [REPOSITORY] - 媒体资源仓储
/// * `desc`: `S3 媒体仓储`
pub struct MediaRepo;

impl MediaRepo {
    //

    ////////

    /// # [REPO] - 根据内部 ID 查询媒体资源
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<MediaEntity>, sqlx::Error> {
        let query = format!(
            r#"
            SELECT {} FROM cola_fs.media
            WHERE id = $1 AND (is_deleted IS NOT TRUE)
            LIMIT 1
            "#,
            MEDIA_COLUMNS
        );

        let entity = sqlx::query_as::<_, MediaEntity>(&query)
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(entity)
    }

    ////////

    /// # [REPO] - 创建媒体资源记录（默认 status = 0 处理中/草稿态）
    pub async fn create(pool: &PgPool, cmd: CreateMediaCmd) -> Result<MediaEntity, sqlx::Error> {
        let now = Utc::now();
        let create_time = now.timestamp();

        let query = format!(
            r#"
            INSERT INTO cola_fs.media (
                _id, app_id, media_type, status, cover_file_id,
                main_file_id, aux_file_id, hls_playlist_url, variants_meta,
                duration, width, height, is_deleted, create_time, created_at, updated_at
            )
            VALUES ($1, $2, $3, 0, $4, $5, $6, $7, $8, $9, $10, $11, false, $12, $13, $14)
            RETURNING {}
            "#,
            MEDIA_COLUMNS
        );

        let entity = sqlx::query_as::<_, MediaEntity>(&query)
            .bind(cmd._id)
            .bind(cmd.app_id)
            .bind(cmd.media_type)
            .bind(cmd.cover_file_id)
            .bind(cmd.main_file_id)
            .bind(cmd.aux_file_id)
            .bind(cmd.hls_playlist_url)
            .bind(cmd.variants_meta)
            .bind(cmd.duration)
            .bind(cmd.width)
            .bind(cmd.height)
            .bind(create_time)
            .bind(now)
            .bind(now)
            .fetch_one(pool)
            .await?;

        Ok(entity)
    }

    ////////

    /// # [REPO] - 更新媒体状态及 HLS / 变体信息（通常由转码成功后异步回调触发）
    pub async fn update_transcode_success(
        pool: &PgPool,
        id: i64,
        hls_playlist_url: Option<&str>,
        variants_meta: Option<&str>,
    ) -> Result<u64, sqlx::Error> {
        let now = Utc::now();

        let query = r#"
            UPDATE cola_fs.media
            SET status = 1,
                hls_playlist_url = COALESCE($1, hls_playlist_url),
                variants_meta = COALESCE($2, variants_meta),
                updated_at = $3
            WHERE id = $4 AND (is_deleted IS NOT TRUE)
        "#;

        let result = sqlx::query(query)
            .bind(hls_playlist_url)
            .bind(variants_meta)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # [REPO] - 【后台管理】分页条件查询媒体列表
    pub async fn admin_find_page(
        pool: &PgPool,
        app_id: Option<&str>,
        media_type: Option<i16>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<MediaEntity>, i64), sqlx::Error> {
        let count_query = r#"
            SELECT COUNT(*) FROM cola_fs.media
            WHERE ($1::text IS NULL OR app_id = $1)
              AND ($2::smallint IS NULL OR media_type = $2)
        "#;

        let total: i64 = sqlx::query_scalar(count_query)
            .bind(app_id)
            .bind(media_type)
            .fetch_one(pool)
            .await?;

        let list_query = format!(
            r#"
            SELECT {} FROM cola_fs.media
            WHERE ($1::text IS NULL OR app_id = $1)
              AND ($2::smallint IS NULL OR media_type = $2)
            ORDER BY id DESC
            LIMIT $3 OFFSET $4
            "#,
            MEDIA_COLUMNS
        );

        let list = sqlx::query_as::<_, MediaEntity>(&list_query)
            .bind(app_id)
            .bind(media_type)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        Ok((list, total))
    }

    ////////

    /// # [REPO] - 逻辑删除媒体记录
    pub async fn delete(pool: &PgPool, id: i64) -> Result<u64, sqlx::Error> {
        let now = Utc::now();

        let query = r#"
            UPDATE cola_fs.media
            SET is_deleted = true, deleted_at = $1, updated_at = $1
            WHERE id = $2 AND (is_deleted IS NOT TRUE)
        "#;

        let result = sqlx::query(query).bind(now).bind(id).execute(pool).await?;

        Ok(result.rows_affected())
    }
}

//////// END
