// repository/src/cola_fs/pg/cnd.rs  -- 仓储 - FS - PG - CDN加速配置仓储
// 2026/8/14 11:59 Created.

////////

use chrono::Utc;
use sqlx::PgPool;
use cola_data::cola_fs::command::cdn::CreateCdnDomainCmd;
use cola_data::cola_fs::entity::cdn::{CdnDomainEntity, CDN_DOMAIN_COLUMNS};

////////

/// # [REPOSITORY] - CDN 加速配置
/// * `desc`: `多应用多桶 CDN 域名绑定与加速策略仓储`
pub struct CdnDomainRepo;

impl CdnDomainRepo {

    /// # [REPO] - 根据内部 ID 查询 CDN 配置
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<CdnDomainEntity>, sqlx::Error> {
        let query = format!(
            r#"
            SELECT {} FROM cola_fs.cdn_domain
            WHERE id = $1 AND (is_deleted IS NOT TRUE)
            LIMIT 1
            "#,
            CDN_DOMAIN_COLUMNS
        );

        let entity = sqlx::query_as::<_, CdnDomainEntity>(&query)
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(entity)
    }

    /// # [REPO] - 根据逻辑桶编码查询对应的 CDN 配置（核心加速路由解析）
    pub async fn find_by_bucket_key(
        pool: &PgPool,
        app_id: Option<&str>,
        bucket_key: &str,
    ) -> Result<Option<CdnDomainEntity>, sqlx::Error> {
        let query = format!(
            r#"
            SELECT {} FROM cola_fs.cdn_domain
            WHERE app_id IS NOT DISTINCT FROM $1
              AND bucket_key = $2
              AND is_enabled = true
              AND (is_deleted IS NOT TRUE)
            LIMIT 1
            "#,
            CDN_DOMAIN_COLUMNS
        );

        let entity = sqlx::query_as::<_, CdnDomainEntity>(&query)
            .bind(app_id)
            .bind(bucket_key)
            .fetch_optional(pool)
            .await?;

        Ok(entity)
    }

    /// # [REPO] - 创建 CDN 域名绑定记录
    pub async fn create(pool: &PgPool, cmd: CreateCdnDomainCmd) -> Result<CdnDomainEntity, sqlx::Error> {
        let now = Utc::now();
        let create_time = now.timestamp();

        let query = format!(
            r#"
            INSERT INTO cola_fs.cdn_domain (
                _id, app_id, bucket_key, cdn_domain, provider,
                is_https, is_enabled, auth_type, auth_key, status,
                is_deleted, create_time, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, true, $7, $8, 1, false, $9, $10, $11)
            RETURNING {}
            "#,
            CDN_DOMAIN_COLUMNS
        );

        let entity = sqlx::query_as::<_, CdnDomainEntity>(&query)
            .bind(cmd._id)
            .bind(cmd.app_id)
            .bind(cmd.bucket_key)
            .bind(cmd.cdn_domain)
            .bind(cmd.provider)
            .bind(cmd.is_https)
            .bind(cmd.auth_type)
            .bind(cmd.auth_key)
            .bind(create_time)
            .bind(now)
            .bind(now)
            .fetch_one(pool)
            .await?;

        Ok(entity)
    }

    /// # [REPO] - 【后台管理】分页条件查询 CDN 配置列表
    pub async fn admin_find_page(
        pool: &PgPool,
        app_id: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<CdnDomainEntity>, i64), sqlx::Error> {
        let count_query = r#"
            SELECT COUNT(*) FROM cola_fs.cdn_domain
            WHERE ($1::text IS NULL OR app_id = $1)
        "#;

        let total: i64 = sqlx::query_scalar(count_query)
            .bind(app_id)
            .fetch_one(pool)
            .await?;

        let list_query = format!(
            r#"
            SELECT {} FROM cola_fs.cdn_domain
            WHERE ($1::text IS NULL OR app_id = $1)
            ORDER BY id DESC
            LIMIT $2 OFFSET $3
            "#,
            CDN_DOMAIN_COLUMNS
        );

        let list = sqlx::query_as::<_, CdnDomainEntity>(&list_query)
            .bind(app_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        Ok((list, total))
    }

    /// # [REPO] - 逻辑删除 CDN 配置
    pub async fn delete(pool: &PgPool, id: i64) -> Result<u64, sqlx::Error> {
        let now = Utc::now();

        let query = r#"
            UPDATE cola_fs.cdn_domain
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
}

//////// END