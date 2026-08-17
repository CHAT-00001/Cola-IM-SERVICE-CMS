// repository/src/cola_fs/pg/cnd.rs  -- 仓储 - FS - PG - CDN加速配置仓储
// 2026/8/14 11:59 Created.

////////

use chrono::Utc;
use sqlx::PgPool;
use cola_data::cola_fs::command::cdn::CreateCdnDomainCmd;
use cola_data::cola_fs::command::cdn::UpdateCdnDomainCmd;
use cola_data::cola_fs::entity::cdn::{CdnDomainEntity, CDN_DOMAIN_COLUMNS};
use tracing::{debug, error, info, warn};

////////

/// # [REPOSITORY] - CDN 加速配置
/// * `desc`: `多应用多桶 CDN 域名绑定与加速策略仓储`
pub struct CdnDomainRepo;

impl CdnDomainRepo {

    /// # [REPO] - 根据内部 ID 查询 CDN 配置
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<Option<CdnDomainEntity>, sqlx::Error> {
        debug!("[🗄️ REPOSITORY][CDN] - 🔍 按 ID 查询 CDN 配置: cdn_id={}", id);

        let query = format!(
            r#"
            SELECT {} FROM cola_fs.cdn_domain
            WHERE id = $1 AND (is_deleted IS NOT TRUE)
            LIMIT 1
            "#,
            CDN_DOMAIN_COLUMNS
        );

        let result = sqlx::query_as::<_, CdnDomainEntity>(&query)
            .bind(id)
            .fetch_optional(pool)
            .await?;

        match result {
            Some(entity) => {
                info!("[🗄️ REPOSITORY][CDN] - ✅️ 按 ID 查询成功: cdn_id={}", id);
                Ok(Some(entity))
            }
            None => {
                warn!("[🗄️ REPOSITORY][CDN] - ⚠️ 按 ID 查询为空: cdn_id={}", id);
                Ok(None)
            }
        }
    }

    ////////

    /// # 2. [REPOSITORY] - 根据应用标识查询启用 CDN 配置
    pub async fn find_by_app_id(
        pool: &PgPool,
        app_id: &str,
    ) -> Result<Option<CdnDomainEntity>, sqlx::Error> {
        debug!(
            "[🗄️ REPOSITORY][CDN] - 🔍 按 app_id 查询 CDN 配置: app_id={:?}",
            app_id
        );

        if app_id.trim().is_empty() {
            warn!(
                "[🗄️ REPOSITORY][CDN] - ⚠️ 收到空 app_id，查询将不会命中: app_id={:?}",
                app_id
            );
        }

        let query = format!(
            r#"
            SELECT {} FROM cola_fs.cdn_domain
            WHERE app_id = $1
              AND is_enabled = true
              AND status = 1
              AND (is_deleted IS NOT TRUE)
            ORDER BY id DESC
            LIMIT 1
            "#,
            CDN_DOMAIN_COLUMNS
        );

        let result = sqlx::query_as::<_, CdnDomainEntity>(&query)
            .bind(app_id)
            .fetch_optional(pool)
            .await;

        match result {
            Ok(Some(entity)) => {
                info!(
                    "[🗄️ REPOSITORY][CDN] - ✅️ 按 app_id 查询成功: app_id={:?}, cdn_id={}",
                    app_id,
                    entity.id
                );
                Ok(Some(entity))
            }
            Ok(None) => {
                warn!(
                    "[🗄️ REPOSITORY][CDN] - ⚠️ 按 app_id 查询为空: app_id={:?}",
                    app_id
                );
                Ok(None)
            }
            Err(error) => {
                error!(
                    "[🗄️ REPOSITORY][CDN] - ❌️ 按 app_id 查询失败: app_id={:?}, error={}",
                    app_id,
                    error
                );
                Err(error)
            }
        }
    }

    /// # [REPO] - 根据逻辑桶编码查询对应的 CDN 配置（核心加速路由解析）
    pub async fn find_by_bucket_key(
        pool: &PgPool,
        app_id: Option<&str>,
        bucket_key: &str,
    ) -> Result<Option<CdnDomainEntity>, sqlx::Error> {
        debug!(
            "[🗄️ REPOSITORY][CDN] - 🔍 按桶查询 CDN 配置: app_id={:?}, bucket_key={:?}",
            app_id,
            bucket_key
        );

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

        let result = sqlx::query_as::<_, CdnDomainEntity>(&query)
            .bind(app_id)
            .bind(bucket_key)
            .fetch_optional(pool)
            .await?;

        match result {
            Some(entity) => {
                info!(
                    "[🗄️ REPOSITORY][CDN] - ✅️ 按桶查询成功: app_id={:?}, bucket_key={:?}, cdn_id={}",
                    app_id,
                    bucket_key,
                    entity.id
                );
                Ok(Some(entity))
            }
            None => {
                warn!(
                    "[🗄️ REPOSITORY][CDN] - ⚠️ 按桶查询为空: app_id={:?}, bucket_key={:?}",
                    app_id,
                    bucket_key
                );
                Ok(None)
            }
        }
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

    ////////

    /// # 5. [REPOSITORY] - 更新 CDN 配置
    pub async fn update(
        pool: &PgPool,
        id: i64,
        cmd: UpdateCdnDomainCmd,
    ) -> Result<Option<CdnDomainEntity>, sqlx::Error> {
        let now = Utc::now();
        let query = format!(
            r#"
            UPDATE cola_fs.cdn_domain
            SET cdn_domain = COALESCE($1, cdn_domain),
                provider = COALESCE($2, provider),
                is_https = COALESCE($3, is_https),
                is_enabled = COALESCE($4, is_enabled),
                auth_type = COALESCE($5, auth_type),
                auth_key = COALESCE($6, auth_key),
                status = COALESCE($7, status),
                updated_at = $8
            WHERE id = $9 AND (is_deleted IS NOT TRUE)
            RETURNING {}
            "#,
            CDN_DOMAIN_COLUMNS
        );

        sqlx::query_as::<_, CdnDomainEntity>(&query)
            .bind(cmd.cdn_domain)
            .bind(cmd.provider)
            .bind(cmd.is_https)
            .bind(cmd.is_enabled)
            .bind(cmd.auth_type)
            .bind(cmd.auth_key)
            .bind(cmd.status)
            .bind(now)
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    ////////

    /// # 6. [REPOSITORY] - 更新 CDN 状态
    pub async fn update_status(
        pool: &PgPool,
        id: i64,
        status: i16,
    ) -> Result<Option<CdnDomainEntity>, sqlx::Error> {
        let now = Utc::now();
        let query = format!(
            r#"
            UPDATE cola_fs.cdn_domain
            SET status = $1, is_enabled = ($1 = 1), updated_at = $2
            WHERE id = $3 AND (is_deleted IS NOT TRUE)
            RETURNING {}
            "#,
            CDN_DOMAIN_COLUMNS
        );

        sqlx::query_as::<_, CdnDomainEntity>(&query)
            .bind(status)
            .bind(now)
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    /// # [REPO] - 【后台管理】分页条件查询 CDN 配置列表
    pub async fn admin_find_page(
        pool: &PgPool,
        app_id: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<(Vec<CdnDomainEntity>, i64), sqlx::Error> {
        debug!(
            "[🗄️ REPOSITORY][CDN] - 🔍 分页查询 CDN 列表: app_id={:?}, limit={}, offset={}",
            app_id,
            limit,
            offset
        );

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

        info!(
            "[🗄️ REPOSITORY][CDN] - ✅️ 分页查询成功: app_id={:?}, count={}, total={}",
            app_id,
            list.len(),
            total
        );

        Ok((list, total))
    }

    /// # [REPO] - 逻辑删除 CDN 配置
    pub async fn delete(pool: &PgPool, id: i64) -> Result<u64, sqlx::Error> {
        debug!("[🗄️ REPOSITORY][CDN] - 🔍 逻辑删除 CDN 配置: cdn_id={}", id);

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

        let affected = result.rows_affected();
        if affected == 0 {
            warn!("[🗄️ REPOSITORY][CDN] - ⚠️ 删除未命中: cdn_id={}", id);
        } else {
            info!("[🗄️ REPOSITORY][CDN] - ✅️ 逻辑删除成功: cdn_id={}", id);
        }

        Ok(affected)
    }
}

//////// END