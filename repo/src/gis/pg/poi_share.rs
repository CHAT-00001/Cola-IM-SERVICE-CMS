// repo/src/gis/pg/share.rs  -- 仓储中心 - GIS - pg - 兴趣点 分享
// 2026/7/6 14:21

////////

use crate::pg_pool;
use cola_data::gis::entity::share::PoiShareEntity;
use sqlx;
use chrono::Utc; // 需要引入 chrono 用于生成时间

////////

/// # [REPOSITORY] - 兴趣点 分享
pub struct PoiShareRepository;

// 构造
impl PoiShareRepository {

    ////////

    /// # 1. [REPOSITORY] - 根据兴趣点ID查找最新的分享记录
    pub async fn find_latest_by_poi_id(poi_id: i64) -> Result<Option<PoiShareEntity>, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_as::<_, PoiShareEntity>(
            "SELECT * FROM cola_gis.poi_share WHERE gis_id = $1 AND is_deleted = 0 ORDER BY create_time DESC LIMIT 1"
        )
            .bind(poi_id)
            .fetch_optional(&pool).await
    }

    ////////

    /// # 2. [REPOSITORY] - 根据兴趣点ID查找热门的分享记录
    pub async fn find_hot_by_poi_id(poi_id: i64, limit: i64) -> Result<Vec<PoiShareEntity>, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_as::<_, PoiShareEntity>(
            "SELECT * FROM cola_gis.poi_share WHERE gis_id = $1 AND is_deleted = 0 ORDER BY share_count DESC LIMIT $2"
        )
            .bind(poi_id)
            .bind(limit)
            .fetch_all(&pool).await
    }

    ////////

    /// # 3. [REPOSITORY] - 根据分享ID 查找单条分享记录(高亮命中)
    pub async fn find_by_share_id(share_id: i64) -> Result<Option<PoiShareEntity>, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_as::<_, PoiShareEntity>(
            "SELECT * FROM cola_gis.poi_share WHERE id = $1 AND is_deleted = 0"
        )
            .bind(share_id)
            .fetch_optional(&pool).await
    }

    ////////

    /// # 4. [REPOSITORY] - 根据兴趣点ID查找最新的分享记录
    // (注：与第1点逻辑类似，如需不同排序或筛选可调整)
    pub async fn list_by_poi_id(poi_id: i64, limit: i64, offset: i64) -> Result<Vec<PoiShareEntity>, sqlx::Error> {
        let pool = pg_pool();
        sqlx::query_as::<_, PoiShareEntity>(
            "SELECT * FROM cola_gis.poi_share WHERE gis_id = $1 AND is_deleted = 0 ORDER BY create_time DESC LIMIT $2 OFFSET $3"
        )
            .bind(poi_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool).await
    }

    ////////

    /// # 5. [REPOSITORY] - 保存分享记录
    pub async fn pg_save_share_record(entity: PoiShareEntity) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        sqlx::query(
            r#"INSERT INTO cola_gis.poi_share
               (user_id, gis_id, target_platform, share_code, sync_id, sync_time, create_time, is_deleted)
               VALUES ($1, $2, $3, $4, $5, $6, $7, 0)
               ON CONFLICT (sync_id) DO NOTHING"#,
        )
            .bind(entity.user_id).bind(entity.poi_id)
            .bind(entity.target_platform)
            .bind(entity.share_code)
            .bind(entity.sync_id)
            .bind(entity.sync_time)
            .bind(entity.created_at)
            .execute(&pool).await?;
        Ok(())
    }

    ////////

    /// # 6. [REPOSITORY] - 根据用户ID查找分享记录
    pub async fn find_share_record_by_user_id(uid: i64, limit: i64, offset: i64) -> Result<Vec<i64>, sqlx::Error> {
        let pool = pg_pool();
        let query = "SELECT gis_id FROM cola_gis.poi_share WHERE user_id = $1 AND is_deleted = 0 ORDER BY create_time DESC LIMIT $2 OFFSET $3";
        sqlx::query_scalar::<_, i64>(query)
            .bind(uid)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool).await
    }

    ////////

    /// # 7. [REPOSITORY] - 用户删除自己的分享记录(支持多条)
    pub async fn soft_delete_user_shares(uid: i64, ids: Vec<i64>) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        sqlx::query(
            "UPDATE cola_gis.poi_share SET is_deleted = 1, deleted_at = $1, del_time = $2 WHERE user_id = $3 AND id = ANY($4)"
        )
            .bind(Utc::now().to_rfc3339())
            .bind(Utc::now().timestamp())
            .bind(uid)
            .bind(ids)
            .execute(&pool).await?;
        Ok(())
    }

    ////////

    /// # 8. [REPOSITORY] - 根据用户ID批量删除分享记录(注销时, 永久封禁时)
    pub async fn soft_delete_all_by_user_id(uid: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        sqlx::query(
            "UPDATE cola_gis.poi_share SET is_deleted = 1, deleted_at = $1, del_time = $2 WHERE user_id = $3"
        )
            .bind(Utc::now().to_rfc3339())
            .bind(Utc::now().timestamp())
            .bind(uid)
            .execute(&pool).await?;
        Ok(())
    }

    ////////

    /// # 9. [REPOSITORY] - 根据Poi同步软删除分享记录(poi删除时)
    pub async fn soft_delete_by_poi_id(poi_id: i64) -> Result<(), sqlx::Error> {
        let pool = pg_pool();
        sqlx::query(
            "UPDATE cola_gis.poi_share SET is_deleted = 1, deleted_at = $1, del_time = $2 WHERE gis_id = $3"
        )
            .bind(Utc::now().to_rfc3339())
            .bind(Utc::now().timestamp())
            .bind(poi_id)
            .execute(&pool).await?;
        Ok(())
    }

    ////////

    /// # 10. [REPOSITORY] - 自动遍历硬删除删除分享记录(定时任务清除失效数据)
    pub async fn hard_delete_expired_shares(before_timestamp: i64) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let result = sqlx::query(
            "DELETE FROM cola_gis.poi_share WHERE is_deleted = 1 AND del_time < $1"
        )
            .bind(before_timestamp)
            .execute(&pool).await?;
        Ok(result.rows_affected())
    }

}

//////// END