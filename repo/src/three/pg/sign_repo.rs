// repo/src/three/pg/sign_repo.rs  -- 仓储 - THREE - 第三方登录配置 PG
// 2026/6/30

////////

use cola_data::three::entity::sign::{ThreeSignEntity, THREE_SIGN_COLUMNS};
use crate::pg_pool;

////////

/// # [REPO] - 第三方登录配置 仓储
pub struct SignRepo;

impl SignRepo {
    //

    ////////

    /// 1. #[REPOSITORY] - 插入（新建）
    pub async fn insert(
        type_id: i64, vendor_id: i64, name: &str,
        client_id: &str, client_secret: &str, redirect_uri: &str, scope: &str,
        config_json: Option<&serde_json::Value>, remark: Option<&str>, status: i16,
    ) -> Result<ThreeSignEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"INSERT INTO three_sign (type_id, vendor_id, name, client_id, client_secret, redirect_uri, scope, config_json, remark, status)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
               RETURNING {}"#,
            THREE_SIGN_COLUMNS
        );
        sqlx::query_as::<_, ThreeSignEntity>(&query)
            .bind(type_id).bind(vendor_id).bind(name)
            .bind(client_id).bind(client_secret).bind(redirect_uri).bind(scope)
            .bind(config_json).bind(remark).bind(status)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// 2. #[REPOSITORY] - 更新（仅改 updated_at，不动 created_at）
    pub async fn update(
        id: i64, type_id: i64, vendor_id: i64, name: &str,
        client_id: &str, client_secret: &str, redirect_uri: &str, scope: &str,
        config_json: Option<&serde_json::Value>, remark: Option<&str>, status: i16,
    ) -> Result<ThreeSignEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"UPDATE three_sign SET type_id=$1, vendor_id=$2, name=$3, client_id=$4, client_secret=$5, redirect_uri=$6, scope=$7, config_json=$8, remark=$9, status=$10, updated_at=NOW()
               WHERE id=$11 RETURNING {}"#,
            THREE_SIGN_COLUMNS
        );
        sqlx::query_as::<_, ThreeSignEntity>(&query)
            .bind(type_id).bind(vendor_id).bind(name)
            .bind(client_id).bind(client_secret).bind(redirect_uri).bind(scope)
            .bind(config_json).bind(remark).bind(status).bind(id)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// 3. #[REPOSITORY] - 按 type_id 查询列表
    pub async fn list_by_type(type_id: i64) -> Result<Vec<ThreeSignEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_sign WHERE type_id = $1 ORDER BY id DESC",
            THREE_SIGN_COLUMNS
        );
        sqlx::query_as::<_, ThreeSignEntity>(&query)
            .bind(type_id)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// 4. #[REPOSITORY] - 按 ID 查询
    pub async fn find_by_id(id: i64) -> Result<Option<ThreeSignEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_sign WHERE id = $1 LIMIT 1",
            THREE_SIGN_COLUMNS
        );
        sqlx::query_as::<_, ThreeSignEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// 5. #[REPOSITORY] - 切换状态
    pub async fn update_status(id: i64, status: i16) -> Result<Option<ThreeSignEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "UPDATE three_sign SET status=$1, updated_at=NOW() WHERE id=$2 RETURNING {}",
            THREE_SIGN_COLUMNS
        );
        sqlx::query_as::<_, ThreeSignEntity>(&query)
            .bind(status).bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// 6. #[REPOSITORY] - 查询所有配置
    pub async fn list_all() -> Result<Vec<ThreeSignEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_sign ORDER BY id DESC",
            THREE_SIGN_COLUMNS
        );
        sqlx::query_as::<_, ThreeSignEntity>(&query)
            .fetch_all(&pool)
            .await
    }
}

////////
