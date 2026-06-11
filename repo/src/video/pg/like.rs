// repo/src/pg/video/like.rs  -- 存储 - PG - 短视频 - 信息
// 2026/6/10 05:25

////////

use crate::pg_pool;
use cola_data::video::command::video::VideoCommand;
use cola_data::video::entity::video::VideoEntity;
use sqlx::{self, Postgres, QueryBuilder};

////////

///
pub struct LikeRepo;

impl LikeRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 保存点赞记录
    // 简化版本 - 使用upsert逻辑
    pub async fn pg_save_video_like(
        uid: i64,
        video_id: i64,
        is_liked: bool,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        INSERT INTO video_like (uid, video_id, is_liked, addtime, updatetime)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (uid, video_id)
        DO UPDATE SET
            is_liked = EXCLUDED.is_liked,
            update_time = EXCLUDED.update_time
    "#;

        let current_timestamp = chrono::Utc::now().timestamp();

        sqlx::query(query)
            .bind(uid)
            .bind(video_id)
            .bind(is_liked)
            .bind(current_timestamp)
            .bind(current_timestamp)
            .execute(&pool)
            .await?;

        Ok(())
    }


    ////////

    /// # 2. [REPOSITORY] - 保存不喜欢记录
    // 简化版本 - 使用upsert逻辑
    pub async fn pg_save_video_unlike(
        uid: i64,
        video_id: i64,
        is_unliked: bool,
    ) -> Result<(), sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
        INSERT INTO video_unlike (uid, video_id, is_unliked, add_time, update_time)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (uid, video_id)
        DO UPDATE SET
            is_unliked = EXCLUDED.is_unliked,
            update_time = EXCLUDED.update_time
    "#;

        let current_timestamp = chrono::Utc::now().timestamp();

        sqlx::query(query)
            .bind(uid)
            .bind(video_id)
            .bind(is_unliked)
            .bind(current_timestamp)
            .bind(current_timestamp)
            .execute(&pool)
            .await?;

        Ok(())
    }



    ////////

    /// # 8. [REPOSITORY] - 获取用户点赞记录的视频 IDs (带分页)
    pub async fn find_like_record_by_user_id(
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        // 🌟 核心修复：
        // 1. 只 SELECT video_id，精准对齐 Vec<i64>
        // 2. 加上 user_id = $1 确保用户隔离
        // 3. 浏览记录一般按“浏览时间（visittime/addtime）”倒序，这里把乱入的 likes 排序去掉了
        let query = "
        SELECT video_id
        FROM video_like
        WHERE user_id = $1 AND status = 1
        ORDER BY add_time DESC
        LIMIT $2 OFFSET $3
    ";

        // 映射单列数据到基础类型，sqlx 内部用 sqlx::query_scalar
        // 或者直接 query 迭代 row.get(0)，但最清爽的是 query_scalar! 宏或直接用单列映射
        sqlx::query_scalar::<_, i64>(query)
            .bind(uid)    // $1
            .bind(limit)  // $2
            .bind(offset) // $3
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 9. [REPOSITORY] - 获取用户不喜欢记录的视频 IDs (带分页)
    pub async fn find_unlike_record_by_user_id(
        uid: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();

        // 🌟 核心修复：
        // 1. 只 SELECT video_id，精准对齐 Vec<i64>
        // 2. 加上 user_id = $1 确保用户隔离
        // 3. 浏览记录一般按“浏览时间（visittime/addtime）”倒序，这里把乱入的 likes 排序去掉了
        let query = "
        SELECT video_id
        FROM video_unlike
        WHERE user_id = $1 AND status = 1
        ORDER BY add_time DESC
        LIMIT $2 OFFSET $3
    ";

        // 映射单列数据到基础类型，sqlx 内部用 sqlx::query_scalar
        // 或者直接 query 迭代 row.get(0)，但最清爽的是 query_scalar! 宏或直接用单列映射
        sqlx::query_scalar::<_, i64>(query)
            .bind(uid)    // $1
            .bind(limit)  // $2
            .bind(offset) // $3
            .fetch_all(&pool)
            .await
    }
}

//////// END
