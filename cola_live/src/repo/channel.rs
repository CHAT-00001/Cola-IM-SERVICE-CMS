// cola_video/src/live/repo/channel.rs  -- 直播频道 - 仓储中心
// 2026-03-11 11:45:00

use sqlx::{PgPool, Postgres, query, query_as};
use crate::live::entity::LiveEntity; // 这里的 Entity 应当包含频道分类字段

/// # 频道分类表字段
/// 对应数据库表 live_channel
const CHANNEL_COLUMNS: &str = r#"
    id,
    icon,
    bg_img,
    name,
    name_en,
    description,
    description_en,
    sort,
    status,
    recommend
"#;

impl Repo {
    /// # REPO - 获取所有启用的频道列表
    /// ## 逻辑：按 sort 升序排列 (数值越小越靠前)，status=1 表示启用
    pub async fn repo_find_channel_list_by_sort(
        pool: &PgPool,
    ) -> Result<Vec<LiveEntity>, sqlx::Error> {
        let sql = format!(
            "SELECT {} FROM live_channel WHERE status = 1 ORDER BY sort ASC",
            CHANNEL_COLUMNS
        );

        sqlx::query_as::<_, LiveEntity>(&sql)
            .fetch_all(pool)
            .await
    }

    /// # REPO - 添加新频道分类
    pub async fn repo_insert_channel_item(
        pool: &PgPool,
        entity: &LiveEntity,
    ) -> Result<i64, sqlx::Error> {
        let sql = r#"
            INSERT INTO live_channel (icon, bg_img, name, name_en, description, description_en, sort, status, recommend)
            VALUES ($1, $2, $3, $4, $5, $6, $7, 1, $8)
            RETURNING id
        "#;

        let row = sqlx::query!(
            sql,
            entity.icon,
            entity.bg_img,
            entity.name,
            entity.name_en,
            entity.description,
            entity.description_en,
            entity.sort,
            entity.recommend
        )
            .fetch_one(pool)
            .await?;

        Ok(row.id)
    }

    /// # REPO - 修改频道信息
    pub async fn repo_update_channel_by_id(
        pool: &PgPool,
        id: i32,
        entity: &LiveEntity,
    ) -> Result<u64, sqlx::Error> {
        let sql = r#"
            UPDATE live_channel
            SET icon = $1, bg_img = $2, name = $3, name_en = $4,
                description = $5, description_en = $6, sort = $7, recommend = $8,
                updated_at = EXTRACT(EPOCH FROM NOW())::BIGINT
            WHERE id = $9
        "#;

        let result = sqlx::query(sql)
            .bind(&entity.icon)
            .bind(&entity.bg_img)
            .bind(&entity.name)
            .bind(&entity.name_en)
            .bind(&entity.description)
            .bind(&entity.description_en)
            .bind(entity.sort)
            .bind(entity.recommend)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    /// # REPO - 禁用/封禁频道
    /// ## 将 status 置为 0，使其在 App 端不可见
    pub async fn repo_ban_channel_by_id(
        pool: &PgPool,
        id: i32,
    ) -> Result<u64, sqlx::Error> {
        let sql = "UPDATE live_channel SET status = 0 WHERE id = $1";

        let result = sqlx::query(sql)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    /// # REPO - 设为推荐频道
    pub async fn repo_set_recommend_status(
        pool: &PgPool,
        id: i32,
        is_recommend: i8,
    ) -> Result<u64, sqlx::Error> {
        let sql = "UPDATE live_channel SET recommend = $1 WHERE id = $2";

        let result = sqlx::query(sql)
            .bind(is_recommend)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }
}