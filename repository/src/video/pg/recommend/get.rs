// repository/src/video/pg/recommend/get.rs
// 仓储 - VIDEO - pg - recommend - get 获取
// 2026/8/2 13:41 Created.

////////

use crate::pg_pool;
use cola_data::video::entity::recommend::recommend::RecommendRecordEntity;

////////

/// # [GET REPOSITORY] - 视频 推荐记录 获取 仓储
pub struct VideoRecommendGetRepository;

// 构造实现
impl VideoRecommendGetRepository {
    //

    ////////

    /// # 1. [REPOSITORY] - 查询用户推荐的视频 IDs (带分页)
    pub async fn pg_find_recommend_ids_by_uid(
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        let query = "
            SELECT video_id
            FROM cola_video.recommend_record
            WHERE user_id = $1 AND status = 1 AND is_deleted = 0
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

    /// # 2. [REPOSITORY] - 根据 user_id 和 video_id 查找单条推荐记录详情
    pub async fn pg_find_record_by_uid_and_video_id(
        uid: i64,
        video_id: i64,
    ) -> Result<Option<RecommendRecordEntity>, sqlx::Error> {
        let pool = pg_pool();

        let query = "
            SELECT *
            FROM cola_video.recommend_record
            WHERE user_id = $1 AND video_id = $2 AND is_deleted = 0
            LIMIT 1
        ";

        sqlx::query_as::<_, RecommendRecordEntity>(query)
            .bind(uid)
            .bind(video_id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// # 3. [REPOSITORY] - 根据 video_id 统计被多少人推荐过（计数用）
    pub async fn pg_count_recommend_by_video_id(video_id: i64) -> Result<i64, sqlx::Error> {
        let pool = pg_pool();

        let query = "
            SELECT COUNT(*)
            FROM cola_video.recommend_record
            WHERE video_id = $1 AND status = 1 AND is_deleted = 0
        ";

        let count: i64 = sqlx::query_scalar(query)
            .bind(video_id)
            .fetch_one(&pool)
            .await?;

        Ok(count)
    }

    ////////

    /// # 4. [REPOSITORY] - 查看某个视频下的所有推荐记录列表 (类似评论列表，带分页)
    pub async fn pg_find_recommend_list_by_video_id(
        video_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<RecommendRecordEntity>, sqlx::Error> {
        let pool = pg_pool();

        let query = "
            SELECT *
            FROM cola_video.recommend_record
            WHERE video_id = $1 AND status = 1 AND is_deleted = 0
            ORDER BY add_time DESC
            LIMIT $2 OFFSET $3
        ";

        sqlx::query_as::<_, RecommendRecordEntity>(query)
            .bind(video_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }
}

//////// END
