// repo/src/pg/video/share.rs  -- 存储 - PG - 短视频 - 分享
// 2026/5/20 20:31 by wx: cestbon10080
// * 1个结构体 + 1个实现块
// * --------

////////

use sqlx::PgPool;
use cola_data::video::entity::share::ShareEntity;

////////

/// # [REPOSITORY] - 短视频分享数据仓库
#[derive(Debug, Clone)]
pub struct VideoShareRepository {
    pool: PgPool, // 💡 将连接池内聚在仓库实体内
}

impl VideoShareRepository {
    /// ## 创建数据仓库实例
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// ## [REPOSITORY] - 保存 - 分享记录
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
}

// * --------
//////// END