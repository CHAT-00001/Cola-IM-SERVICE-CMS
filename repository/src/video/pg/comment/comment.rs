// repository/src/new/pg/file/file.rs  --
// 仓储 - VIDEO - pg - file - 评论
// 2026/6/8 16:55

////////

use crate::pg_pool;
use cola_data::cola_video::command::comment::CommentCommand;
use cola_data::cola_video::entity::comment::VideoCommentEntity;
use sqlx::{self, Postgres, QueryBuilder};

////////

// 数据表原始字段
const COMMENT_COLUMNS: &str = r#"
    id, uuid, show_id, user_id, video_id, parent_id, content, at_uids,
    thumb_url, photos_url,video_url, voice_url, lat, lng,
    likes, steps, collects, reply, visibility, region_code,
    status, deleted_at, deleted_by, addtime, created_at, updated_at
"#;

// 局部辅助结构体：用来承接带有“动态计算距离”的数据库返回行
#[derive(Debug, sqlx::FromRow)]
pub struct VideoHomeRow {
    #[sqlx(flatten)] // 自动把标准字段映射进 Entity
    pub entity: VideoCommentEntity,
    #[sqlx(default)]
    pub distance: Option<f64>, // 承接动态计算的距离
}

/// # 搜索排序规则枚举（新增：最新发布）
#[derive(Debug, Clone, Copy)]
pub enum SearchOrder {
    Distance,  // 距离最近 (默认)
    MostViews, // 播放量最多
    MostLikes, // 点赞量最多
    Latest,    // 最新发布
}

/// # [REPOSITORY] - 评论 仓储
pub struct CommentRepo;

impl CommentRepo {
    //

    ////////

    /// # 3. [REPOSITORY] - 最新
    /// * `desc`: 根据视频ID查找最新的评论列表（按发布时间戳 add_time 降序）
    pub async fn find_new_comments_by_video_id(
        video_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoCommentEntity>, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "SELECT {}
     FROM video_comments
     WHERE status = 1
       AND video_id = $1
     ORDER BY add_time DESC
     LIMIT $2 OFFSET $3",
            COMMENT_COLUMNS
        );

        sqlx::query_as::<_, VideoCommentEntity>(&query)
            .bind(video_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    /// # 4. [REPOSITORY] - 热门
    /// * `desc`: 根据视频ID查找热门的评论列表（主排序：点赞量降序，次排序：发布时间戳降序）
    pub async fn find_hot_comments_by_video_id(
        video_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoCommentEntity>, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "SELECT {}
     FROM video_comments
     WHERE status = 1
       AND video_id = $1
     ORDER BY likes DESC, add_time DESC
     LIMIT $2 OFFSET $3",
            COMMENT_COLUMNS
        );

        sqlx::query_as::<_, VideoCommentEntity>(&query)
            .bind(video_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 5. [REPOSITORY] - 我发布的评论
    /// * `desc`: 根据用户ID（uid）查找自己发布的最新评论列表
    pub async fn find_comments_by_user_id(
        user_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoCommentEntity>, sqlx::Error> {
        let pool = pg_pool();

        // 💡 提示：如果你的评论表里，用户ID的物理字段名就叫 `uid`，请把下面的 `user_id = $1` 改为 `uid = $1`
        let query = format!(
            "SELECT {}
     FROM video_comments
     WHERE status = 1
       AND uid = $1
     ORDER BY add_time DESC
     LIMIT $2 OFFSET $3",
            COMMENT_COLUMNS
        );

        sqlx::query_as::<_, VideoCommentEntity>(&query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 6. [REPOSITORY] - 回复我的
    /// * `desc`: 查找回复我的评论
    pub async fn find_reply_me_comments(
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoCommentEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM new WHERE status = 1 ORDER BY RANDOM() LIMIT $1 OFFSET $2",
            COMMENT_COLUMNS
        );

        sqlx::query_as::<_, VideoCommentEntity>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 7. [REPOSITORY] - 更新评论点赞（幂等）
    pub async fn update_comment_like_by_id(
        uid: i64,
        comment_id: i64,
        is_liked: bool,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        if is_liked {
            sqlx::query(
                r#"
            INSERT INTO video_comments_like (uid, comment_id, created_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (uid, comment_id)
            DO NOTHING
            "#,
            )
            .bind(uid)
            .bind(comment_id)
            .execute(&pool)
            .await?;
        } else {
            sqlx::query(
                r#"
            DELETE FROM video_comments_like
            WHERE uid = $1 AND comment_id = $2
            "#,
            )
            .bind(uid)
            .bind(comment_id)
            .execute(&pool)
            .await?;
        }

        Ok(())
    }

    /// # 8. [REPOSITORY] - 更新不喜欢
    pub async fn update_comment_unlike_by_id(
        uid: Option<i64>,
        comment_id: i64,
        is_unliked: bool,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        if is_unliked {
            sqlx::query(
                r#"
            INSERT INTO video_comments_unlike (uid, comment_id, created_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (uid, comment_id)
            DO NOTHING
            "#,
            )
            .bind(uid)
            .bind(comment_id)
            .execute(&pool)
            .await?;
        } else {
            sqlx::query(
                r#"
            DELETE FROM video_comments_unlike
            WHERE uid = $1 AND comment_id = $2
            "#,
            )
            .bind(uid)
            .bind(comment_id)
            .execute(&pool)
            .await?;
        }

        Ok(())
    }

    ////////

    /// # 9. [REPOSITORY] - 根据 IDs 集合批量查找视频列表 (保持高性能)
    pub async fn find_by_ids(ids: &[i64]) -> Result<Vec<VideoCommentEntity>, sqlx::Error> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM new WHERE id = ANY($1) AND status = 1",
            COMMENT_COLUMNS
        );

        sqlx::query_as::<_, VideoCommentEntity>(&query)
            .bind(ids)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 10. [REPOSITORY] - 保存视频评论
    /// * `user_id` 用户 ID
    pub async fn save_comment_by_video_id(
        user_id: i64,        // 用户 ID
        video_id: i64,       //  视频 ID
        cmd: CommentCommand, // 评论命令
        visibility: i16,     // 可见性
    ) -> Result<VideoCommentEntity, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "INSERT INTO video_comment (user_id, video_id, parent_id, content, visibility, status) \
             VALUES ($1, $2, $3, $4, $5, 1) \
             RETURNING {}",
            COMMENT_COLUMNS
        );

        sqlx::query_as::<_, VideoCommentEntity>(&query)
            .bind(user_id)
            .bind(video_id)
            .bind(cmd.parent_id)
            .bind(cmd.content) // 👈 简介字段安全入库
            .bind(visibility) // 👈 风控计算后的可见性状态
            .fetch_one(&pool)
            .await
    }

    /// # 11. [REPOSITORY] - 软删除评论（更新状态为已删除）
    /// * `user_id`: 用户ID（用于验证权限）
    /// * `comment_id`: 评论ID
    /// * 返回更新后的评论信息
    pub async fn user_del_comment_by_id(
        uid: i64,
        comment_id: i64,
    ) -> Result<VideoCommentEntity, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "UPDATE video_comment
         SET status = 0,  -- 0=已删除, 1=正常
             updated_at = NOW()
         WHERE id = $1 AND uid = $2
         RETURNING {}",
            COMMENT_COLUMNS
        );

        sqlx::query_as::<_, VideoCommentEntity>(&query)
            .bind(comment_id)
            .bind(uid)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 12. [REPOSITORY] - 管理员根据评论ID删除评论
    /// * `comment_id`: 评论ID
    pub async fn admin_del_comment_by_id(
        comment_id: i64,
    ) -> Result<VideoCommentEntity, sqlx::Error> {
        let pool = pg_pool();

        let query = format!(
            "DELETE FROM video_comment
         WHERE id = $1
         RETURNING {}",
            COMMENT_COLUMNS
        );

        sqlx::query_as::<_, VideoCommentEntity>(&query)
            .bind(comment_id)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// # 12. [REPOSITORY] - 视频删除时同步软删除该视频下的所有评论
    /// * `video_id`: 视频 ID
    /// * 更新评论状态为已删除，便于后续恢复或审计
    pub async fn sync_soft_del_comments_by_video_id(video_id: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        UPDATE video_comment
        SET status = 0,
            deleted_at = NOW(),
            deleted_by = -1  -- -1 表示系统自动删除（视频被删）
        WHERE video_id = $1 AND status = 1
    "#;

        let result = sqlx::query(query).bind(video_id).execute(&pool).await?;

        Ok(result.rows_affected())
    }
}

//////// END
