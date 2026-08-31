// repository/src/cola_fs/pg/bucket.rs  -- 仓储 - FS - PG - 存储桶仓储
// 2026/8/14 13:10

////////

use chrono::{DateTime, Utc};
use cola_data::cola_fs::command::bucket::CreateBucketCmd;
use cola_data::cola_fs::entity::bucket::{BUCKET_COLUMNS, BucketEntity};
use sqlx::PgPool;

////////

/// # [REPOSITORY] - 存储桶
/// * `desc`: `S3 存储桶配置管理仓储`
pub struct BucketRepo;

impl BucketRepo {
    //

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

    /// # 3. [REPOSITORY] - 检查应用 ID 是否已被其他存储桶占用
    pub async fn exists_by_app_id(
        pool: &PgPool,
        app_id: &str,
        exclude_id: Option<i64>,
    ) -> Result<bool, sqlx::Error> {
        let query = r#"
            SELECT EXISTS(
                SELECT 1
                FROM cola_fs.bucket
                WHERE app_id = $1
                  AND ($2::BIGINT IS NULL OR id <> $2)
                  AND is_deleted IS NOT TRUE
            )
        "#;

        sqlx::query_scalar(query)
            .bind(app_id)
            .bind(exclude_id)
            .fetch_one(pool)
            .await
    }

    ////////

    /// # [REPO] - 根据应用标识与逻辑桶编码查询存储桶
    pub async fn find_by_key(
        pool: &PgPool,
        app_id: Option<&str>,
        bucket: &str,
    ) -> Result<Option<BucketEntity>, sqlx::Error> {
        let query = format!(
            r#"
            SELECT {} FROM cola_fs.bucket
            WHERE app_id IS NOT DISTINCT FROM $1
              AND bucket = $2
              AND (is_deleted IS NOT TRUE)
            LIMIT 1
            "#,
            BUCKET_COLUMNS
        );

        let entity = sqlx::query_as::<_, BucketEntity>(&query)
            .bind(app_id)
            .bind(bucket)
            .fetch_optional(pool)
            .await?;

        Ok(entity)
    }

    ////////

    /// # [REPO] - 创建存储桶映射记录
    pub async fn create(pool: &PgPool, cmd: CreateBucketCmd) -> Result<BucketEntity, sqlx::Error> {
        let now = Utc::now();
        let query = format!(
            r#"
            INSERT INTO cola_fs.bucket (
                _id, app_id, type_id, vendor_id, name, bucket, cdn_domain,
                access_key, secret_key, endpoint, region, config_json,
                remark, status, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7,
                    $8, $9, $10, $11, $12, $13, $14, $15, $16)
            RETURNING {}
            "#,
            BUCKET_COLUMNS
        );

        let entity = sqlx::query_as::<_, BucketEntity>(&query)
            .bind(cmd._id)
            .bind(cmd.app_id)
            .bind(cmd.type_id)
            .bind(cmd.vendor_id)
            .bind(cmd.name)
            .bind(cmd.bucket)
            .bind(cmd.cdn_domain)
            .bind(cmd.access_key)
            .bind(cmd.secret_key)
            .bind(cmd.endpoint)
            .bind(cmd.region)
            .bind(cmd.config_json)
            .bind(cmd.remark)
            .bind(cmd.status)
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

        let result = sqlx::query(query).bind(now).bind(id).execute(pool).await?;

        Ok(result.rows_affected())
    }

    ////////

    /// # [REPO] - 创建存储桶 (wrapper 方法)
    pub async fn create_bucket(
        _bucket: String,
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
        // 1. 构建基础条件（支持按 app_id 过滤，支持按名称或 bucket 模糊搜索）
        // 后台视角默认不强制过滤 is_deleted 和 status，但提供灵活的条件组合
        let kw = keyword.map(|s| format!("%{}%", s));

        // 查询符合条件的总数
        let count_query = r#"
            SELECT COUNT(*) FROM cola_fs.bucket
            WHERE ($1::text IS NULL OR app_id = $1)
              AND ($2::text IS NULL OR name ILIKE $2 OR bucket::text ILIKE $2)
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
              AND ($2::text IS NULL OR name ILIKE $2 OR bucket::text ILIKE $2)
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
