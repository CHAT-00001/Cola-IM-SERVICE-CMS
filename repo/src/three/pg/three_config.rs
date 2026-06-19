// repo/src/three/pg/three_config.rs  -- 仓储 - THREE - 配置 PG
// 2026/6/18

//////

use cola_data::three::entity::three_config::{ThreeConfigEntity, THREE_CONFIG_COLUMNS};
use crate::pg_pool;

//////

/// # [REPO] - 配置 仓储
pub struct ConfigRepo;

impl ConfigRepo {

    /// 1. 插入
    pub async fn insert(
        type_id: i64, vendor_id: i64, name: &str, bucket: &str,
        access_key: &str, secret_key: &str, endpoint: &str, region: &str,
        config_json: Option<&serde_json::Value>, remark: Option<&str>, status: i16,
    ) -> Result<ThreeConfigEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"INSERT INTO three_config (type_id, vendor_id, name, bucket, access_key, secret_key, endpoint, region, config_json, remark, status)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
               RETURNING {}"#,
            THREE_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeConfigEntity>(&query)
            .bind(type_id).bind(vendor_id).bind(name).bind(bucket)
            .bind(access_key).bind(secret_key).bind(endpoint).bind(region)
            .bind(config_json).bind(remark).bind(status)
            .fetch_one(&pool)
            .await
    }

    /// 2. 更新
    pub async fn update(
        id: i64, type_id: i64, vendor_id: i64, name: &str, bucket: &str,
        access_key: &str, secret_key: &str, endpoint: &str, region: &str,
        config_json: Option<&serde_json::Value>, remark: Option<&str>, status: i16,
    ) -> Result<ThreeConfigEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"UPDATE three_config SET type_id=$1, vendor_id=$2, name=$3, bucket=$4, access_key=$5, secret_key=$6, endpoint=$7, region=$8, config_json=$9, remark=$10, status=$11, updated_at=NOW()
               WHERE id=$12 RETURNING {}"#,
            THREE_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeConfigEntity>(&query)
            .bind(type_id).bind(vendor_id).bind(name).bind(bucket)
            .bind(access_key).bind(secret_key).bind(endpoint).bind(region)
            .bind(config_json).bind(remark).bind(status).bind(id)
            .fetch_one(&pool)
            .await
    }

    /// 3. 按 type_id 查询列表
    pub async fn list_by_type(type_id: i64) -> Result<Vec<ThreeConfigEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_config WHERE type_id = $1 ORDER BY id DESC",
            THREE_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeConfigEntity>(&query)
            .bind(type_id)
            .fetch_all(&pool)
            .await
    }

    /// 4. 按 ID 查询
    pub async fn find_by_id(id: i64) -> Result<Option<ThreeConfigEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_config WHERE id = $1 LIMIT 1",
            THREE_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeConfigEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }

    /// 5. 按 biz_module + biz_type 查绑定配置（JOIN）
    pub async fn find_binded(biz_module: &str, biz_type: &str) -> Result<Option<ThreeConfigEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"SELECT c.{}
               FROM three_config c
               INNER JOIN three_biz_binding b ON b.three_config_id = c.id
               WHERE b.biz_module = $1 AND b.biz_type = $2 AND b.status = 1 AND c.status = 1
               LIMIT 1"#,
            THREE_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeConfigEntity>(&query)
            .bind(biz_module)
            .bind(biz_type)
            .fetch_optional(&pool)
            .await
    }
}
