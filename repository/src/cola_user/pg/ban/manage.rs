// repository/src/cola_user/pg/ban/manage.rs
// 仓储 - 可乐用户 - pg - 封禁 - 管理
// 2026/5/23 05:07

////////

use crate::pg_pool;
use cola_data::cola_user::entity::user::{USER_COLUMNS, UserEntity};
use sqlx::{self};

////////

/// # [MANAGE REPOSITORY] - 管理员
/// * `desc`: `用户封禁管理仓储`
pub struct UserBanManageRepo;

impl UserBanManageRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 管理列表
    /// * `desc`: `管理员查看用户综合列表（支持多条件组合筛选）`
    /// * `condition`: `⚠️ 管理员身份`
    pub async fn find_manage_users_list(
        user_id: Option<i64>,    // 用户ID (可选)
        city_id: Option<String>, // 城市ID
        keyword: Option<String>, // 关键词(昵称/个签)
        start_time: Option<i64>, // 开始时间
        end_time: Option<i64>,   // 结束时间
        status: Option<i16>,     // 状态码
        limit: i64,              // 数量
        offset: i64,             // 偏移量
    ) -> Result<Vec<UserEntity>, sqlx::Error> {
        let pool = pg_pool();

        // 使用 SQL 参数与 COALESCE/NULL 逻辑实现安全且灵活的可选条件复合查询
        let query = format!(
            r#"
            SELECT {} FROM "cola_user"."ban"
            WHERE ($1::BIGINT IS NULL OR id = $1)
              AND ($2::TEXT IS NULL OR city_id = $2)
              AND ($3::TEXT IS NULL OR (user_nickname ILIKE '%' || $3 || '%' OR signature ILIKE '%' || $3 || '%'))
              AND ($4::BIGINT IS NULL OR create_time >= $4)
              AND ($5::BIGINT IS NULL OR create_time <= $5)
              AND ($6::SMALLINT IS NULL OR status = $6)
            ORDER BY create_time DESC, id DESC
            LIMIT $7 OFFSET $8
            "#,
            USER_COLUMNS
        );

        sqlx::query_as::<_, UserEntity>(&query)
            .bind(user_id)
            .bind(city_id)
            .bind(keyword)
            .bind(start_time)
            .bind(end_time)
            .bind(status)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await
    }

    ////////

    /// # 2. [REPOSITORY] - 单个操作
    /// * `desc`: `管理员更新单个用户状态码`
    /// * `condition`: `⚠️ 管理员身份`
    pub async fn single_set_status_code_by_user_id(
        user_id: i64,            // 目标用户ID
        code: i16,               // 新状态码
        _reason: Option<String>, // 原因（预留）
    ) -> Result<u16, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
            UPDATE "cola_user"."ban"
            SET status = $1, updated_at = NOW()
            WHERE id = $2
        "#;

        let result = sqlx::query(query)
            .bind(code)
            .bind(user_id)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected() as u16)
    }

    ////////

    /// # 3. [REPOSITORY] - 批量操作
    /// * `desc`: `管理员批量更新用户状态码`
    /// * `condition`: `⚠️ 管理员身份`
    pub async fn batch_set_status_code_by_user_ids(
        user_ids: &[i64],        // 目标用户IDs
        code: i16,               // 新状态码
        _reason: Option<String>, // 原因（预留）
    ) -> Result<u16, sqlx::Error> {
        let pool = pg_pool();

        let query = r#"
            UPDATE "cola_user"."ban"
            SET status = $1, updated_at = NOW()
            WHERE id = ANY($2)
        "#;

        let result = sqlx::query(query)
            .bind(code)
            .bind(user_ids)
            .execute(&pool)
            .await?;

        Ok(result.rows_affected() as u16)
    }
}

//////// END
