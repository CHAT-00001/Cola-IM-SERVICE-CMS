// repository/src/cola_fs/pg/file_repo  -- 仓储 - FS - PG - 文件
// 2026/6/30 18:03

////////

use chrono::Utc;
use cola_data::cola_fs::entity::file::{FsFileEntity, FS_FILE_COLUMNS};
use crate::pg_pool;

////////

/// # [REPO] - 文件存储配置 仓储
pub struct FileRepo;

impl FileRepo {
    //

    ////////

    /// 1. #[REPOSITORY] - 插入（新建：created_at & updated_at）
    pub async fn insert(
        type_id: i64, vendor_id: i64, name: &str, bucket: &str,
        access_key: &str, secret_key: &str, endpoint: &str, region: &str,
        config_json: Option<&serde_json::Value>, remark: Option<&str>, status: i16,
    ) -> Result<FsFileEntity, sqlx::Error> {
        let pool = pg_pool();
        let now = Utc::now();
        let query = format!(
            r#"INSERT INTO cola_fs.file (type_id, vendor_id, name, bucket, access_key, secret_key, endpoint, region, config_json, remark, status, created_at, updated_at)
               VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13)
               RETURNING {}"#,
            FS_FILE_COLUMNS
        );
        sqlx::query_as::<_, FsFileEntity>(&query)
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
    ) -> Result<FsFileEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            r#"UPDATE cola_fs.file SET type_id=$1, vendor_id=$2, name=$3, bucket=$4, access_key=$5, secret_key=$6, endpoint=$7, region=$8, config_json=$9, remark=$10, status=$11, updated_at=NOW()
               WHERE id=$12 RETURNING {}"#,
            FS_FILE_COLUMNS
        );
        sqlx::query_as::<_, FsFileEntity>(&query)
            .bind(type_id).bind(vendor_id).bind(name).bind(bucket)
            .bind(access_key).bind(secret_key).bind(endpoint).bind(region)
            .bind(config_json).bind(remark).bind(status).bind(id)
            .fetch_one(&pool)
            .await
    }

    ////////

    /// 3. #[REPOSITORY] - 按 type_id 查询列表
    pub async fn list_by_type(type_id: i64) -> Result<Vec<FsFileEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_fs.file WHERE type_id = $1 ORDER BY id DESC",
            FS_FILE_COLUMNS
        );
        sqlx::query_as::<_, FsFileEntity>(&query)
            .bind(type_id)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// 4. #[REPOSITORY] - 按 ID 查询
    pub async fn find_by_id(id: i64) -> Result<Option<FsFileEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_fs.file WHERE id = $1 LIMIT 1",
            FS_FILE_COLUMNS
        );
        sqlx::query_as::<_, FsFileEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// 5. #[REPOSITORY] - 切换状态（前端快速启用/禁用）
    pub async fn update_status(id: i64, status: i16) -> Result<Option<FsFileEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "UPDATE cola_fs.file SET status=$1, updated_at=NOW() WHERE id=$2 RETURNING {}",
            FS_FILE_COLUMNS
        );
        sqlx::query_as::<_, FsFileEntity>(&query)
            .bind(status).bind(id)
            .fetch_optional(&pool)
            .await
    }

    ////////

    /// 6. #[REPOSITORY] - 查询所有配置
    pub async fn list_all() -> Result<Vec<FsFileEntity>, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_fs.file ORDER BY id DESC",
            FS_FILE_COLUMNS
        );
        sqlx::query_as::<_, FsFileEntity>(&query)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// 7. #[REPOSITORY] - 检查文件是否存在
    pub async fn check_file_exists(file_id: i64) -> Result<bool, sqlx::Error> {
        let pool = pg_pool();
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM cola_fs.file WHERE id = $1"
        )
        .bind(file_id)
        .fetch_one(&pool)
        .await?;
        Ok(result > 0)
    }

    ////////

    /// 8. #[REPOSITORY] - 检查文件是否可用（status=1）
    pub async fn check_file_available(file_id: i64) -> Result<bool, sqlx::Error> {
        let pool = pg_pool();
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM cola_fs.file WHERE id = $1 AND status = 1"
        )
        .bind(file_id)
        .fetch_one(&pool)
        .await?;
        Ok(result > 0)
    }

    ////////

    /// 9. #[REPOSITORY] - 检查用户是否拥有文件
    pub async fn check_file_owner(uid: i64, file_id: i64) -> Result<bool, sqlx::Error> {
        let pool = pg_pool();
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM cola_fs.file WHERE id = $1 AND vendor_id = $2"
        )
        .bind(file_id)
        .bind(uid)
        .fetch_one(&pool)
        .await?;
        Ok(result > 0)
    }

    ////////

    /// 10. #[REPOSITORY] - 删除文件
    pub async fn delete_file(_uid: i64, _file_id: i64) -> Result<u64, sqlx::Error> {
        todo!("delete_file - 未实现")
    }

    ////////

    /// 11. #[REPOSITORY] - 批量删除文件
    pub async fn batch_delete_files(_uid: i64, _file_ids: Vec<i64>) -> Result<u64, sqlx::Error> {
        todo!("batch_delete_files - 未实现")
    }

    ////////

    /// 12. #[REPOSITORY] - 按 ID 获取文件
    pub async fn get_file_by_id(_file_id: i64) -> Result<Option<FsFileEntity>, sqlx::Error> {
        todo!("get_file_by_id - 未实现")
    }

    ////////

    /// 13. #[REPOSITORY] - 按 object_key 获取文件
    pub async fn get_file_by_object_key(_object_key: &str) -> Result<Option<FsFileEntity>, sqlx::Error> {
        todo!("get_file_by_object_key - 未实现")
    }

    ////////

    /// 14. #[REPOSITORY] - 批量获取文件
    pub async fn batch_get_files(_file_ids: Vec<i64>) -> Result<Vec<FsFileEntity>, sqlx::Error> {
        todo!("batch_get_files - 未实现")
    }

    ////////

    /// 15. #[REPOSITORY] - 列表查询用户文件
    pub async fn list_user_files(_uid: i64, _limit: i64, _offset: i64) -> Result<Vec<FsFileEntity>, sqlx::Error> {
        todo!("list_user_files - 未实现")
    }

    ////////

    /// 16. #[REPOSITORY] - 列表查询应用文件
    pub async fn list_app_files(_app_id: &str, _limit: i64, _offset: i64) -> Result<Vec<FsFileEntity>, sqlx::Error> {
        todo!("list_app_files - 未实现")
    }

    ////////

    /// 17. #[REPOSITORY] - 标记文件为正式
    pub async fn mark_files_as_official(_uid: i64, _file_ids: Vec<i64>) -> Result<u64, sqlx::Error> {
        todo!("mark_files_as_official - 未实现")
    }

    ////////

    /// 18. #[REPOSITORY] - 更新文件元数据
    pub async fn update_file_metadata(_file_id: i64, _new_name: Option<String>) -> Result<FsFileEntity, sqlx::Error> {
        todo!("update_file_metadata - 未实现")
    }

    ////////

    /// 19. #[REPOSITORY] - 更新文件状态
    pub async fn update_file_status(_file_id: i64, _status: i16) -> Result<u64, sqlx::Error> {
        todo!("update_file_status - 未实现")
    }

    ////////

    /// 20. #[REPOSITORY] - 统计用户文件数量
    pub async fn stat_user_file_count(_uid: i64) -> Result<i64, sqlx::Error> {
        todo!("stat_user_file_count - 未实现")
    }

    ////////

    /// 21. #[REPOSITORY] - 统计用户存储使用量
    pub async fn stat_user_storage_used(_uid: i64) -> Result<i64, sqlx::Error> {
        todo!("stat_user_storage_used - 未实现")
    }
}

////////
