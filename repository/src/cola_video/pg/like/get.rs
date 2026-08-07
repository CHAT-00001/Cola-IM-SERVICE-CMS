// repository/src/cola_video/pg/like/get.rs
// 仓储 - VIDEO - pg - like - get 点赞/不喜欢
// 2026/8/2 14:49 Created.

////////

use crate::pg_pool;
use cola_data::cola_video::entity::like::dislike::VideoDislikeEntity;
use cola_data::cola_video::entity::like::like::VideoLikeEntity;
use sqlx::{self, Postgres};
////////

/// [GET REPOSITORY] - 视频 点赞/不喜欢 获取 仓储
pub struct VideoLikeGetRepo;

impl VideoLikeGetRepo {
    ////////

    /// # 1. [REPOSITORY] - 统计某个视频的点赞总数
    pub async fn count_video_likes(video_id: i64) -> Result<i64, sqlx::Error> {
        let pool = pg_pool();
        let query = "
            SELECT COUNT(*)
            FROM cola_video.like
            WHERE video_id = $1 AND status = 1 AND is_requested = false
        ";

        let count: i64 = sqlx::query_scalar(query)
            .bind(video_id)
            .fetch_one(&pool)
            .await?;

        Ok(count)
    }

    ////////

    /// # 2. [REPOSITORY] - 统计某个视频的不喜欢总数
    pub async fn count_video_dislikes(video_id: i64) -> Result<i64, sqlx::Error> {
        let pool = pg_pool();
        let query = "
            SELECT COUNT(*)
            FROM cola_video.dislike
            WHERE video_id = $1 AND status = 1 AND is_deleted = false
        ";

        let count: i64 = sqlx::query_scalar(query)
            .bind(video_id)
            .fetch_one(&pool)
            .await?;

        Ok(count)
    }

    ////////

    /// # 3. [REPOSITORY] - 获取用户点赞记录的视频 IDs (带分页)
    pub async fn find_like_record_by_user_id(
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();
        let query = "
            SELECT video_id
            FROM cola_video.like
            WHERE uid = $1 AND status = 1 AND is_deleted = false
            ORDER BY add_time DESC
            LIMIT $2 OFFSET $3
        ";

        sqlx::query_scalar::<_, i64>(query)
            .bind(uid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 4. [REPOSITORY] - 获取用户不喜欢记录的视频 IDs (带分页)
    pub async fn find_unlike_record_by_user_id(
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();
        let query = "
            SELECT video_id
            FROM cola_video.dislike
            WHERE uid = $1 AND status = 1 AND is_deleted = false
            ORDER BY add_time DESC
            LIMIT $2 OFFSET $3
        ";

        sqlx::query_scalar::<_, i64>(query)
            .bind(uid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 5. [REPOSITORY] - 根据视频 ID 查找对应的点赞记录列表 (带分页)
    pub async fn find_likes_by_video_id(
        video_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoLikeEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = "
            SELECT *
            FROM cola_video.like
            WHERE video_id = $1 AND status = 1 AND is_deleted = false
            ORDER BY add_time DESC
            LIMIT $2 OFFSET $3
        ";

        sqlx::query_as::<_, VideoLikeEntity>(query)
            .bind(video_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 6. [REPOSITORY] - 根据视频 ID 查找对应的不喜欢记录列表 (带分页)
    pub async fn find_dislikes_by_video_id(
        video_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<VideoDislikeEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = "
            SELECT *
            FROM cola_video.dislike
            WHERE video_id = $1 AND status = 1 AND is_deleted = false
            ORDER BY add_time DESC
            LIMIT $2 OFFSET $3
        ";

        sqlx::query_as::<_, VideoDislikeEntity>(query)
            .bind(video_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }
}

//////// END
