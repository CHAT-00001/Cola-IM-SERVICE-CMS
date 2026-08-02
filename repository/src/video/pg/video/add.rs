// repository/src/video/pg/video/add.rs  --
// 仓储 - VIDEO - pg - video - add
// 2026/6/10 19:52

////////

use crate::pg_pool;
use chrono::Utc;
use cola_data::video::command::video::edit::VideoUpdateCommand;
use cola_data::video::command::video::new::VideoNewCommand;
use cola_data::video::command::video::permission::VideoUpdatePermissionCommand;
use cola_data::video::entity::video::video::{VIDEO_COLUMNS, VideoEntity};
use sqlx::{self, Postgres, QueryBuilder};

////////

/// [ADD REPOSITORY] - 发布视频
pub struct AddRepository;

impl AddRepository {
    //

    ////////

    /// # 1. [REPOSITORY] - 保存
    /// * `desc`: `用户发布视频落库`
    pub async fn pg_save_video_by_uid(
        uid: i64,
        cmd: VideoNewCommand,
        visibility: i16,
    ) -> Result<VideoEntity, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "INSERT INTO cola_video.video (uid, title, description, href, visibility, status) \
             VALUES ($1, $2, $3, $4, $5, 1) \
             RETURNING {}",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(uid)
            .bind(cmd.title)
            .bind(cmd.description) // 👈 简介字段安全入库
            .bind(cmd.href)
            .bind(visibility) // 👈 风控计算后的可见性状态
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 编辑内容
    /// * `desc`: `用户编辑视频落库`
    pub async fn update_content_by_video_id(
        video_id: i64,
        cmd: VideoUpdateCommand,
    ) -> Result<VideoEntity, sqlx::Error> {
        let pool = pg_pool();
        let now = Utc::now();

        let query = format!(
            "UPDATE cola_video.video \
         SET title = $2, description = $3, thumbnail = $4, updated_at = $5 \
         WHERE video_id = $1 \
         RETURNING {}",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(video_id)
            .bind(cmd.title)
            .bind(cmd.description)
            .bind(cmd.cover_url)
            .bind(now)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 修改权限
    /// * `desc`: `用户修改视频权限落库`
    pub async fn update_permission_by_video_id(
        video_id: i64,
        cmd: VideoUpdatePermissionCommand,
    ) -> Result<VideoEntity, sqlx::Error> {
        let pool = pg_pool();
        let now = Utc::now();

        let query = format!(
            "UPDATE cola_video.video \
         SET visibility_perm = $2, comment_perm = $3, danmaku_perm = $4, collect_perm = $5, download_perm = $6, updated_at = $7 \
         WHERE video_id = $1 \
         RETURNING {}",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(video_id)
            .bind(cmd.visibility_perm)
            .bind(cmd.comment_perm)
            .bind(cmd.danmaku_perm)
            .bind(cmd.collect_perm)
            .bind(cmd.download_perm)
            .bind(now)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 单个软删除
    /// * `desc` :用户删除视频落库
    pub async fn pg_delete_video_by_id(video_id: i64) -> Result<VideoEntity, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        UPDATE cola_video.video
        SET
            is_del = 1,
            del_time = EXTRACT(EPOCH FROM NOW())::BIGINT,
            deleted_at = NOW()
        WHERE video_id = $1 AND is_del = 0
        RETURNING video_id, title, description, href, visibility, status,
                  is_del, del_time, deleted_at, created_at, updated_at
    "#;

        sqlx::query_as::<_, VideoEntity>(query)
            .bind(video_id)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 批量软删除
    /// * `desc` :用户删除视频落库
    pub async fn pg_delete_video_by_ids(video_ids: Vec<i64>) -> Result<VideoEntity, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        UPDATE cola_video.video
        SET
            is_del = 1,
            del_time = EXTRACT(EPOCH FROM NOW())::BIGINT,
            deleted_at = NOW()
        WHERE video_id = $1 AND is_del = 0
        RETURNING video_id, title, description, href, visibility, status,
                  is_del, del_time, deleted_at, created_at, updated_at
    "#;

        sqlx::query_as::<_, VideoEntity>(query)
            .bind(video_ids)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 12. [REPOSITORY] - 同步更新视频弹幕数量（减指定数量）
    /// * `video_id`: 视频 ID
    /// * `count`: 删除的弹幕数量
    /// * 返回更新后的弹幕数量
    pub async fn sync_decrement_danmaku_count_by_num(
        video_id: i64,
        count: i64,
    ) -> Result<i64, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        UPDATE cola_video.video
        SET danmaku_count = GREATEST(danmaku_count - $1, 0),
            updated_at = NOW()
        WHERE id = $2
        RETURNING danmaku_count
    "#;

        let danmaku_count: i64 = sqlx::query_scalar(query)
            .bind(count)
            .bind(video_id)
            .fetch_one(&pool)
            .await?;

        Ok(danmaku_count)
    }

    ////////

    /// # 8. [REPOSITORY] - 查找某个用户发布的视频列表
    pub async fn find_new_list_by_user_id(
        user_id: i64,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<VideoEntity>, sqlx::Error> {
        let pool = pg_pool();

        // 使用参数化查询，避免 SQL 注入
        let query = format!(
            "SELECT {} FROM cola_video.video WHERE uid = $1 AND status = 1 OFFSET $2 LIMIT $3",
            VIDEO_COLUMNS
        );

        sqlx::query_as::<_, VideoEntity>(&query)
            .bind(user_id)
            .bind(offset)
            .bind(limit)
            .fetch_all(&pool) // 使用 fetch_all 获取多条记录
            .await
    }
}

//////// END
