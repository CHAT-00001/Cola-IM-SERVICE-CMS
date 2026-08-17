// repository/src/cola_fs/pg/bucket.rs  -- 仓储 - FS - PG - 存储桶仓储
// 2026/8/14 13:10

////////

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use cola_data::cola_fs::command::bucket::CreateBucketCmd;
use cola_data::cola_fs::entity::bucket::{BucketEntity, BUCKET_COLUMNS};

////////

/// # [REPOSITORY] - 存储桶
/// * `desc`: `S3 存储桶配置管理仓储`
pub struct BucketRepo;

impl BucketRepo {

    ////////

    /// # [REPO] - 根据内部 ID 查询存储桶
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<BucketEntity>, sqlx::Error> {

        let query = format!(
            r#"
            SELECT {} FROM cola_fs.bucket
            WHERE id = $1 AND (is_deleted IS NOT TRUE)
            LIMIT 1
            "#,
            BUCKET_COLUMNS
        );

        let entity = sqlx::query_as::<_, BucketEntity>(&query)
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(entity)
    }

    ////////

    /// # 2. [REPOSITORY] - 根据应用标识查询启用存储桶
    pub async fn find_by_app_id(
        pool: &PgPool,
        app_id: &str,
    ) -> Result<Option<BucketEntity>, sqlx::Error> {
        let query = format!(
            r#"
            SELECT {} FROM cola_fs.bucket
            WHERE app_id = $1
              AND status = 1
              AND (is_deleted IS NOT TRUE)
            ORDER BY id DESC
            LIMIT 1
            "#,
            BUCKET_COLUMNS
        );

        sqlx::query_as::<_, BucketEntity>(&query)
            .bind(app_id)
            .fetch_optional(pool)
            .await
    }

    ////////

    /// # [REPO] - 根据应用标识与逻辑桶编码查询存储桶
    pub async fn find_by_key(
        pool: &PgPool,
        app_id: Option<&str>,
        bucket_key: &str,
    ) -> Result<Option<BucketEntity>, sqlx::Error> {
        let query = format!(
            r#"
            SELECT {} FROM cola_fs.bucket
            WHERE app_id IS NOT DISTINCT FROM $1
              AND bucket_key = $2
              AND (is_deleted IS NOT TRUE)
            LIMIT 1
            "#,
            BUCKET_COLUMNS
        );

        let entity = sqlx::query_as::<_, BucketEntity>(&query)
            .bind(app_id)
            .bind(bucket_key)
            .fetch_optional(pool)
            .await?;

        Ok(entity)
    }

    ////////

    /// # [REPO] - 创建存储桶映射记录
    pub async fn create(pool: &PgPool, cmd: CreateBucketCmd) -> Result<BucketEntity, sqlx::Error> {
        let now = Utc::now();
        let create_time = now.timestamp();

        let query = format!(
            r#"
            INSERT INTO cola_fs.bucket (
                _id, app_id, bucket_key, name, cdn_domain, provider, s3_bucket,
                s3_region, s3_endpoint, access_key, secret_key,
                is_public, upload_policy, status, is_deleted,
                create_time, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, 1, false, $14, $15, $16)
            RETURNING {}
            "#,
            BUCKET_COLUMNS
        );

        let entity = sqlx::query_as::<_, BucketEntity>(&query)
            .bind(cmd._id)
            .bind(cmd.app_id)
            .bind(cmd.bucket_key)
            .bind(cmd.name)
            .bind(cmd.cdn_domain)
            .bind(cmd.provider)
            .bind(cmd.s3_bucket)
            .bind(cmd.s3_region)
            .bind(cmd.s3_endpoint)
            .bind(cmd.access_key)
            .bind(cmd.secret_key)
            .bind(cmd.is_public)
            .bind(cmd.upload_policy)
            .bind(create_time)
            .bind(now)
            .bind(now)
            .fetch_one(pool)
            .await?;

        Ok(entity)
    }

    ////////

    /// # [REPO] - 逻辑删除存储桶
    pub async fn delete(pool: &PgPool, id: i64) -> Result<u64, sqlx::Error> {
        let now = Utc::now();

        let query = r#"
            UPDATE cola_fs.bucket
            SET is_deleted = true, deleted_at = $1, updated_at = $1
            WHERE id = $2 AND (is_deleted IS NOT TRUE)
        "#;

        let result = sqlx::query(query)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # [REPO] - 创建存储桶 (wrapper 方法)
    pub async fn create_bucket(
        _bucket_key: String,
        _name: String,
        _provider: String,
        _s3_bucket: String,
        _s3_region: String,
        _s3_endpoint: String,
        _access_key: String,
        _secret_key: String,
    ) -> Result<BucketEntity, sqlx::Error> {
        // TODO: 实现存储桶创建逻辑
        todo!("create_bucket - 未实现")
    }

    ////////

    /// # [REPO] - 【后台管理】分页条件查询存储桶列表（无视状态与逻辑删除限制，支持关键字与应用筛选）
    pub async fn admin_find_page(
        pool: &PgPool,
        app_id: Option<&str>,
        keyword: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<BucketEntity>, i64), sqlx::Error> {
        // 1. 构建基础条件（支持按 app_id 过滤，支持按名称或 bucket_key 模糊搜索）
        // 后台视角默认不强制过滤 is_deleted 和 status，但提供灵活的条件组合
        let kw = keyword.map(|s| format!("%{}%", s));

        // 查询符合条件的总数
        let count_query = r#"
            SELECT COUNT(*) FROM cola_fs.bucket
            WHERE ($1::text IS NULL OR app_id = $1)
              AND ($2::text IS NULL OR name ILIKE $2 OR bucket_key ILIKE $2)
        "#;

        let total: i64 = sqlx::query_scalar(count_query)
            .bind(app_id)
            .bind(kw.as_deref())
            .fetch_one(pool)
            .await?;

        // 查询当前页数据
        let list_query = format!(
            r#"
            SELECT {} FROM cola_fs.bucket
            WHERE ($1::text IS NULL OR app_id = $1)
              AND ($2::text IS NULL OR name ILIKE $2 OR bucket_key ILIKE $2)
            ORDER BY id DESC
            LIMIT $3 OFFSET $4
            "#,
            BUCKET_COLUMNS
        );

        let list = sqlx::query_as::<_, BucketEntity>(&list_query)
            .bind(app_id)
            .bind(kw.as_deref())
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        Ok((list, total))
    }
}

//////// END