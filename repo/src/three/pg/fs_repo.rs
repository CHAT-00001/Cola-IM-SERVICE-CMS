// repo/src/three/pg/fs_repo.rs  -- 仓储 - THREE - 文件存储配置 PG
// 2026/6/30

////////

use chrono::Utc;
use cola_data::three::entity::fs::{FileStorageEntity, THREE_FS_COLUMNS};
use crate::pg_pool;

////////

/// # [REPO] - 文件存储配置 仓储
pub struct FsRepo;

impl FsRepo {
    //

    ////////

    /// 1. #[REPOSITORY] - 插入（新建：created_at & updated_at）
    pub async fn insert(
        type_id: i64, vendor_id: i64, name: &str, bucket: &str,
        access_key: &str, secret_key: &str, endpoint: &str, region: &str,
        config_json: Option<&serde_json::Value>, remark: Option<&str>, status: i16,
    ) -> Result<FileStorageEntity, sqlx::Error> {
        let pool = pg_pool();
        let now = Utc::now();
        let query = format!(
            r#"INSERT INTO three_fs (type_id, vendor_id, name, bucket, access_key, secret_key, endpoint, region, config_json, remark, status, created_at, updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)
               RETURNING {}"#,
            THREE_FS_COLUMNS
        );
        sqlx::query_as::<_, FileStorageEntity>(&query)
            .bind(type_id).bind(vendor_id).bind(name).bind(bucket)
            .bind(access_key).bind(secret_key).bind(endpoint).bind(region)
            .bind(config_json).bind(remark).bind(status)
            .bind(now).bind(now)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// 2. #[REPOSITORY] - 更新（仅改 updated_at，不动 created_at）
    pub async fn update(
        id: i64, type_id: i64, vendor_id: i64, name: &str, bucket: &str,
        access_key: &str, secret_key: &str, endpoint: &str, region: &str,
        config_json: Option<&serde_json::Value>, remark: Option<&str>, status: i16,
    ) -> Result<FileStorageEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"UPDATE three_fs SET type_id=$1, vendor_id=$2, name=$3, bucket=$4, access_key=$5, secret_key=$6, endpoint=$7, region=$8, config_json=$9, remark=$10, status=$11, updated_at=NOW()
               WHERE id=$12 RETURNING {}"#,
            THREE_FS_COLUMNS
        );
        sqlx::query_as::<_, FileStorageEntity>(&query)
            .bind(type_id).bind(vendor_id).bind(name).bind(bucket)
            .bind(access_key).bind(secret_key).bind(endpoint).bind(region)
            .bind(config_json).bind(remark).bind(status).bind(id)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// 3. #[REPOSITORY] - 按 type_id 查询列表
    pub async fn list_by_type(type_id: i64) -> Result<Vec<FileStorageEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_fs WHERE type_id = $1 ORDER BY id DESC",
            THREE_FS_COLUMNS
        );
        sqlx::query_as::<_, FileStorageEntity>(&query)
            .bind(type_id)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// 4. #[REPOSITORY] - 按 ID 查询
    pub async fn find_by_id(id: i64) -> Result<Option<FileStorageEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_fs WHERE id = $1 LIMIT 1",
            THREE_FS_COLUMNS
        );
        sqlx::query_as::<_, FileStorageEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// 5. #[REPOSITORY] - 切换状态（前端快速启用/禁用）
    pub async fn update_status(id: i64, status: i16) -> Result<Option<FileStorageEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "UPDATE three_fs SET status=$1, updated_at=NOW() WHERE id=$2 RETURNING {}",
            THREE_FS_COLUMNS
        );
        sqlx::query_as::<_, FileStorageEntity>(&query)
            .bind(status).bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// 6. #[REPOSITORY] - 查询所有配置
    pub async fn list_all() -> Result<Vec<FileStorageEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM three_fs ORDER BY id DESC",
            THREE_FS_COLUMNS
        );
        sqlx::query_as::<_, FileStorageEntity>(&query)
            .fetch_all(&pool)
            .await
    }
}

////////
