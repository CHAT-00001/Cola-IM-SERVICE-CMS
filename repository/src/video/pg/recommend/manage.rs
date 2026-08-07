// repository/src/video/pg/recommend/manage.rs
// 仓储 - VIDEO - pg - recommend - manage 管理
// 2026/8/2 13:42 Created.

////////

use crate::pg_pool;
use cola_data::video::command::recommend::RecommendCommand;

////////

/// # [ADD REPOSITORY] - 视频 推荐 管理 仓储
pub struct VideoRecommendManageRepository;

// 构造实现
impl VideoRecommendManageRepository {
    //

    ////////

    /// # 1. [REPOSITORY] - 保存推荐记录
    pub async fn save_recommend_record(
        uid: i64,
        video_id: i64,
        cmd: &RecommendCommand,
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();

        let now = chrono::Utc::now();
        let timestamp = now.timestamp();
        let datetime = now.naive_utc();

        // 1. 修正：VALUES 必须对应 5 个参数
        let query = "
        INSERT INTO cola_video.recommend (user_id, video_id, remark, add_time, created_at)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (user_id, video_id) DO NOTHING
    ";

        // 2. 修正：确保 bind 的顺序与 $1 到 $5 完全一致
        let result = sqlx::query(query)
            .bind(uid)
            .bind(video_id)
            .bind(&cmd.remark) // 对应 $3
            .bind(timestamp) // 对应 $4
            .bind(datetime) // 对应 $5
            .execute(&pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # 2. [REPOSITORY] - 查询用户点赞记录的视频 IDs (带分页)
    pub async fn find_recommend_record_by_user_id(
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
        ORDER BY addtime DESC
        LIMIT $2 OFFSET $3
    ";

        // 映射单列数据到基础类型，sqlx 内部用 sqlx::query_scalar
        // 或者直接 query 迭代 row.get(0)，但最清爽的是 query_scalar! 宏或直接用单列映射
        sqlx::query_scalar::<_, i64>(query)
            .bind(uid) // $1
            .bind(limit) // $2
            .bind(offset) // $3
            .fetch_all(&pool)
            .await
    }
}

//////// END
