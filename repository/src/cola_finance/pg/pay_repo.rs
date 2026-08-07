// repository/src/cola_finance/pg/pay_repo.rs  -- 仓储 - 钱包 - pg - 支付
// 2026/7/27 18:59

////////

use cola_data::wallet::entity::pay::{ThreePayConfigEntity, THREE_PAY_CONFIG_COLUMNS};
use crate::pg_pool;

////////

/// # [REPO] - 支付配置 仓储
pub struct PayRepo;

impl PayRepo {
    //

    ////////

    /// 1. #[REPOSITORY] - 插入（新建）
    pub async fn insert(
        type_id: i64, vendor_id: i64, name: &str,
        mch_id: &str, api_key: &str, notify_url: &str,
        config_json: Option<&serde_json::Value>, remark: Option<&str>, status: i16,
    ) -> Result<ThreePayConfigEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"INSERT INTO cola_wallet.pay (type_id, vendor_id, name, mch_id, api_key, notify_url, config_json, remark, status)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
               RETURNING {}"#,
            THREE_PAY_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreePayConfigEntity>(&query)
            .bind(type_id).bind(vendor_id).bind(name)
            .bind(mch_id).bind(api_key).bind(notify_url)
            .bind(config_json).bind(remark).bind(status)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// 2. #[REPOSITORY] - 更新（仅改 updated_at，不动 created_at）
    pub async fn update(
        id: i64, type_id: i64, vendor_id: i64, name: &str,
        mch_id: &str, api_key: &str, notify_url: &str,
        config_json: Option<&serde_json::Value>, remark: Option<&str>, status: i16,
    ) -> Result<ThreePayConfigEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"UPDATE cola_wallet.pay SET type_id=$1, vendor_id=$2, name=$3, mch_id=$4, api_key=$5, notify_url=$6, config_json=$7, remark=$8, status=$9, updated_at=NOW()
               WHERE id=$10 RETURNING {}"#,
            THREE_PAY_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreePayConfigEntity>(&query)
            .bind(type_id).bind(vendor_id).bind(name)
            .bind(mch_id).bind(api_key).bind(notify_url)
            .bind(config_json).bind(remark).bind(status).bind(id)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// 3. #[REPOSITORY] - 按 type_id 查询列表
    pub async fn list_by_type(type_id: i64) -> Result<Vec<ThreePayConfigEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_wallet.pay WHERE type_id = $1 ORDER BY id DESC",
            THREE_PAY_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreePayConfigEntity>(&query)
            .bind(type_id)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// 4. #[REPOSITORY] - 按 ID 查询
    pub async fn find_by_id(id: i64) -> Result<Option<ThreePayConfigEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_wallet.pay WHERE id = $1 LIMIT 1",
            THREE_PAY_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreePayConfigEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// 5. #[REPOSITORY] - 切换状态
    pub async fn update_status(id: i64, status: i16) -> Result<Option<ThreePayConfigEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "UPDATE cola_wallet.pay SET status=$1, updated_at=NOW() WHERE id=$2 RETURNING {}",
            THREE_PAY_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreePayConfigEntity>(&query)
            .bind(status).bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// 6. #[REPOSITORY] - 查询所有配置
    pub async fn list_all() -> Result<Vec<ThreePayConfigEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_wallet.pay ORDER BY id DESC",
            THREE_PAY_CONFIG_COLUMNS
        );
        sqlx::query_as::<_, ThreePayConfigEntity>(&query)
            .fetch_all(&pool)
            .await
    }
}

//////// END
