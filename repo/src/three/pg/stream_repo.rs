// repo/src/three/pg/stream_repo.rs  -- 仓储 - THREE - 直播推流配置 PG
// 2026/6/30 05:01

////////
 
use cola_data::three::entity::stream::{ThreeStreamEntity, STREAM_CONFIG_COLUMNS};
use crate::pg_pool;

////////

/// # [REPO] - 推流配置 仓储
pub struct StreamRepo;

impl StreamRepo {
    //

    ////////

    /// 1. #[REPOSITORY] - 插入（新建）
    pub async fn insert(
        type_id: i64, vendor_id: i64, name: &str, push_domain: &str,
        access_key: &str, secret_key: &str, expire_seconds: i32,
        config_json: Option<&serde_json::Value>, remark: Option<&str>, status: i16,
    ) -> Result<ThreeStreamEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"INSERT INTO three_stream (type_id, vendor_id, name, push_domain, access_key, secret_key, expire_seconds, config_json, remark, status)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
               RETURNING {}"#,
            STREAM_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeStreamEntity>(&query)
            .bind(type_id).bind(vendor_id).bind(name).bind(push_domain)
            .bind(access_key).bind(secret_key).bind(expire_seconds)
            .bind(config_json).bind(remark).bind(status)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// 2. #[REPOSITORY] - 更新（仅改 updated_at，不动 created_at）
    pub async fn update(
        id: i64, type_id: i64, vendor_id: i64, name: &str, push_domain: &str,
        access_key: &str, secret_key: &str, expire_seconds: i32,
        config_json: Option<&serde_json::Value>, remark: Option<&str>, status: i16,
    ) -> Result<ThreeStreamEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"UPDATE three_stream SET type_id=$1, vendor_id=$2, name=$3, push_domain=$4, access_key=$5, secret_key=$6, expire_seconds=$7, config_json=$8, remark=$9, status=$10, updated_at=NOW()
               WHERE id=$11 RETURNING {}"#,
            STREAM_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeStreamEntity>(&query)
            .bind(type_id).bind(vendor_id).bind(name).bind(push_domain)
            .bind(access_key).bind(secret_key).bind(expire_seconds)
            .bind(config_json).bind(remark).bind(status).bind(id)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// 3. #[REPOSITORY] - 按 type_id 查询列表
    pub async fn list_by_type(type_id: i64) -> Result<Vec<ThreeStreamEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_stream WHERE type_id = $1 ORDER BY id DESC",
            STREAM_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeStreamEntity>(&query)
            .bind(type_id)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// 4. #[REPOSITORY] - 按 ID 查询
    pub async fn find_by_id(id: i64) -> Result<Option<ThreeStreamEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_stream WHERE id = $1 LIMIT 1",
            STREAM_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeStreamEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// 5. #[REPOSITORY] - 切换状态
    pub async fn update_status(id: i64, status: i16) -> Result<Option<ThreeStreamEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "UPDATE three_stream SET status=$1, updated_at=NOW() WHERE id=$2 RETURNING {}",
            STREAM_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeStreamEntity>(&query)
            .bind(status).bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// 6. #[REPOSITORY] - 查询所有配置
    pub async fn list_all() -> Result<Vec<ThreeStreamEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_stream ORDER BY id DESC",
            STREAM_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeStreamEntity>(&query)
            .fetch_all(&pool)
            .await
    }
}

////////
