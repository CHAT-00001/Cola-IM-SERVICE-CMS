// repository/src/cola_gis/pg/count  -- 仓储 - GIS - pg - 兴趣点 收藏记录
// 2026/7/6 14:19

////////

use crate::pg_pool;
use chrono::Utc;
use cola_data::cola_gis::command::collect::PoiCollectCommand;
use sqlx; // 引入用于时间处理

////////

/// # [POI REPOSITORY] - 兴趣点 收藏
pub struct PoiCollectRepo;

// 构造实现
impl PoiCollectRepo {
    // BUILD

    ////////

    /// # 1. [REPOSITORY] - ✅️ 保存兴趣点收藏记录(入库)
    pub async fn save_collect_by_gis_id(
        uid: i64,
        gis_id: i64,
        cmd: &PoiCollectCommand,
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let now = Utc::now();
        // 补齐: 初始写入时 is_deleted 默认为 0
        let query = "
            INSERT INTO cola_gis.poi_collect (user_id, gis_id, remark, add_time, created_at, is_deleted)
            VALUES ($1, $2, $3, $4, $5, 0)
            ON CONFLICT (user_id, gis_id) DO UPDATE SET is_deleted = 0, remark = $3, add_time = $4";
        let result = sqlx::query(query)
            .bind(uid)
            .bind(gis_id)
            .bind(&cmd.remark)
            .bind(now.timestamp())
            .bind(now.naive_utc())
            .execute(&pool)
            .await?;
        Ok(result.rows_affected())
    }

    ////////

    /// # 2. [REPOSITORY] - ❌️ 用户软删除兴趣点收藏记录 (失效)
    pub async fn delete_collect_by_gis_id(uid: i64, gis_id: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let now = Utc::now();
        // 软删除模式
        let query = "UPDATE cola_gis.poi_collect SET is_deleted = 1, deleted_at = $1, del_time = $2
                     WHERE user_id = $3 AND gis_id = $4 AND is_deleted = 0";
        let result = sqlx::query(query)
            .bind(now.naive_utc())
            .bind(now.timestamp())
            .bind(uid)
            .bind(gis_id)
            .execute(&pool)
            .await?;
        Ok(result.rows_affected())
    }

    ////////

    /// # 3. [REPOSITORY] - ▶ 👤  根据用户ID查找TA的收藏记录
    pub async fn find_collect_ids_by_user_id(
        user_id: i64,
        keyword: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();
        // 补齐: 增加 is_deleted = 0 的过滤条件
        let mut sql =
            "SELECT gis_id FROM cola_gis.poi_collect WHERE user_id = $1 AND is_deleted = 0"
                .to_string();
        if let Some(ref kw) = keyword {
            if !kw.is_empty() {
                sql.push_str(" AND remark ILIKE $2");
            }
        }
        sql.push_str(" ORDER BY add_time DESC LIMIT $3 OFFSET $4");
        let mut query = sqlx::query_scalar::<_, i64>(&sql);
        query = query.bind(user_id);
        if let Some(ref kw) = keyword {
            if !kw.is_empty() {
                query = query.bind(format!("%{}%", kw));
            } else {
                query = query.bind("");
            }
        }
        query = query.bind(limit).bind(offset);
        query.fetch_all(&pool).await
    }

    ////////

    /// # 4. [REPOSITORY] - ❌️ 👤  根据用户ID软删除TA的收藏记录(用户注销/永封/删除时)
    pub async fn soft_delete_all_by_user_id(uid: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let now = Utc::now();
        let query = "UPDATE cola_gis.poi_collect SET is_deleted = 1, deleted_at = $1, del_time = $2
                     WHERE user_id = $3 AND is_deleted = 0";
        let result = sqlx::query(query)
            .bind(now.naive_utc())
            .bind(now.timestamp())
            .bind(uid)
            .execute(&pool)
            .await?;
        Ok(result.rows_affected())
    }

    ////////

    /// # 6. [REPOSITORY] - ❌️ 📍  根据兴趣点ID软删除TA的收藏记录(兴趣点失效/删除时)
    pub async fn soft_delete_all_by_gis_id(gis_id: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let now = Utc::now();
        let query = "UPDATE cola_gis.poi_collect SET is_deleted = 1, deleted_at = $1, del_time = $2
                     WHERE gis_id = $3 AND is_deleted = 0";
        let result = sqlx::query(query)
            .bind(now.naive_utc())
            .bind(now.timestamp())
            .bind(gis_id)
            .execute(&pool)
            .await?;
        Ok(result.rows_affected())
    }

    ////////

    /// # 7. [REPOSITORY] - ❌️ ⏰️  定时任务自动硬删除失效过期的收藏记录
    pub async fn hard_delete_expired_collects(before_timestamp: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        // 彻底从数据库中移除
        let query = "DELETE FROM cola_gis.poi_collect WHERE is_deleted = 1 AND del_time < $1";
        let result = sqlx::query(query)
            .bind(before_timestamp)
            .execute(&pool)
            .await?;
        Ok(result.rows_affected())
    }
}

//////// END
