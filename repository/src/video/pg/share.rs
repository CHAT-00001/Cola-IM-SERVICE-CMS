// repository/src/pg/video/share.rs  -- 存储 - PG - 短视频 - 分享
// 2026/5/20 20:31

////////

use sqlx::PgPool;
use cola_data::video::entity::share::ShareEntity;
use crate::pg_pool;
////////

/// # [REPOSITORY] - 短视频分享数据仓库
#[derive(Debug, Clone)]
pub struct ShareRepository {
    pool: PgPool, // 💡 将连接池内聚在仓库实体内
}

impl ShareRepository {
    /// # 创建数据仓库实例
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// # [REPOSITORY] - 保存 - 分享记录
    /// * 防重刷（基于 sync_id 的强幂等机制）
    pub async fn pg_save_share_record(&self, entity: ShareEntity) -> Result<(), sqlx::Error> {
        // 💡 使用更加稳健、不依赖编译期数据库连接的 sqlx::query 动态绑定
        sqlx::query(
            r#"
            INSERT INTO video_share
            (user_id, video_id, target_platform, share_code, sync_id, sync_time, create_time)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (sync_id) DO NOTHING
            "#,
        )
            .bind(entity.user_id)
            .bind(entity.video_id)
            .bind(entity.target_platform)
            .bind(entity.share_code)
            .bind(entity.sync_id) // sqlx 会自动将 uuid::Uuid 映射为 Postgres 的 UUID 物理类型
            .bind(entity.sync_time)
            .bind(entity.create_time)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    ////////

    /// # 8. [REPOSITORY] - 获取用户点分享记录的视频 IDs (带分页)
    pub async fn find_share_record_by_user_id(
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
        FROM video_share
        WHERE user_id = $1 AND status = 1
        ORDER BY addtime DESC
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

// * --------
//////// END