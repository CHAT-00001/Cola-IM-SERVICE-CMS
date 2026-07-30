// repository/src/three/pg/sms_repo.rs  -- 仓储 - THREE - 短信配置 PG
// 2026/6/30

////////

use cola_data::three::entity::sms::{ThreeSmsConfigEntity, THREE_SMS_CONFIG_COLUMNS};
use crate::pg_pool;

////////
 
/// # [REPO] - 短信配置 仓储
pub struct SmsRepo;

impl SmsRepo {
    //

    ////////

    /// 1. #[REPOSITORY] - 插入（新建）
    pub async fn insert(
        type_id: i64, vendor_id: i64, name: &str,
        access_key: &str, secret_key: &str, endpoint: &str, region: &str,
        sign_name: &str,
        config_json: Option<&serde_json::Value>, remark: Option<&str>, status: i16,
    ) -> Result<ThreeSmsConfigEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"INSERT INTO three_sms (type_id, vendor_id, name, access_key, secret_key, endpoint, region, sign_name, config_json, remark, status)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
               RETURNING {}"#,
            THREE_SMS_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeSmsConfigEntity>(&query)
            .bind(type_id).bind(vendor_id).bind(name)
            .bind(access_key).bind(secret_key).bind(endpoint).bind(region)
            .bind(sign_name)
            .bind(config_json).bind(remark).bind(status)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// 2. #[REPOSITORY] - 更新（仅改 updated_at，不动 created_at）
    pub async fn update(
        id: i64, type_id: i64, vendor_id: i64, name: &str,
        access_key: &str, secret_key: &str, endpoint: &str, region: &str,
        sign_name: &str,
        config_json: Option<&serde_json::Value>, remark: Option<&str>, status: i16,
    ) -> Result<ThreeSmsConfigEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"UPDATE three_sms SET type_id=$1, vendor_id=$2, name=$3, access_key=$4, secret_key=$5, endpoint=$6, region=$7, sign_name=$8, config_json=$9, remark=$10, status=$11, updated_at=NOW()
               WHERE id=$12 RETURNING {}"#,
            THREE_SMS_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeSmsConfigEntity>(&query)
            .bind(type_id).bind(vendor_id).bind(name)
            .bind(access_key).bind(secret_key).bind(endpoint).bind(region)
            .bind(sign_name)
            .bind(config_json).bind(remark).bind(status).bind(id)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// 3. #[REPOSITORY] - 按 type_id 查询列表
    pub async fn list_by_type(type_id: i64) -> Result<Vec<ThreeSmsConfigEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_sms WHERE type_id = $1 ORDER BY id DESC",
            THREE_SMS_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeSmsConfigEntity>(&query)
            .bind(type_id)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// 4. #[REPOSITORY] - 按 ID 查询
    pub async fn find_by_id(id: i64) -> Result<Option<ThreeSmsConfigEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_sms WHERE id = $1 LIMIT 1",
            THREE_SMS_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeSmsConfigEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// 5. #[REPOSITORY] - 切换状态
    pub async fn update_status(id: i64, status: i16) -> Result<Option<ThreeSmsConfigEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "UPDATE three_sms SET status=$1, updated_at=NOW() WHERE id=$2 RETURNING {}",
            THREE_SMS_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeSmsConfigEntity>(&query)
            .bind(status).bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// 6. #[REPOSITORY] - 查询所有配置
    pub async fn list_all() -> Result<Vec<ThreeSmsConfigEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_sms ORDER BY id DESC",
            THREE_SMS_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreeSmsConfigEntity>(&query)
            .fetch_all(&pool)
            .await
    }
}

////////
