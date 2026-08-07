// repository/src/cola_user/pg/black/list.rs
// 仓储层 - 可乐用户 - pg - 黑名单 - 列表仓储
// 2026/8/7 03:22 Created.

////////

use crate::pg_pool;
use app_config::GLOBAL_DB;
use cola_data::cola_user::entity::black::{UserBlackEntity, USER_BLACK_COLUMNS};
use sqlx::{self, PgPool};

////////

/// # [LIST REPOSITORY] - 获取
/// * `desc`: `用户黑名单审计日志列表仓储`
pub struct UserBlackListRepo;

// 构造实现
impl UserBlackListRepo {
    //

    ////////

    /// # [REPOSITORY] - 多条件组合筛选获取黑名单审计日志列表
    /// * `desc`: 支持 actor_id、target_id、起止时间筛选，并返回总数与实体列表
    /// * `sort`: id 降序
    pub async fn find_black_record_list(
        actor_id: Option<i64>,   // 操作者UID (谁)
        target_id: Option<i64>,  // 目标用户UID (拉黑了谁)
        start_time: Option<i64>, // 开始时间戳
        end_time: Option<i64>,   // 截止时间戳
        limit: i64,              // 数量
        offset: i64,             // 偏移量
    ) -> Result<(i64, Vec<UserBlackEntity>), sqlx::Error> {
        let pool = pg_pool();

        // 1. 查询符合条件的总数 (Total Count)
        let count_query = r#"
            SELECT COUNT(*)
            FROM "cola_user.black"
            WHERE ($1 IS NULL OR uid = $1)
              AND ($2 IS NULL OR user_id = $2)
              AND ($3 IS NULL OR add_time >= $3)
              AND ($4 IS NULL OR add_time <= $4)
              AND is_deleted = false
        "#;

        let total: i64 = sqlx::query_scalar(count_query)
            .bind(actor_id)
            .bind(target_id)
            .bind(start_time)
            .bind(end_time)
            .fetch_one(&pool)
            .await?;

        // 如果总数为 0，直接返回空列表，避免无效的分页查询
        if total == 0 {
            return Ok((0, vec![]));
        }

        // 2. 查询分页数据列表
        let list_query = format!(
            r#"
            SELECT {}
            FROM "cola_user.black"
            WHERE ($1 IS NULL OR uid = $1)
              AND ($2 IS NULL OR user_id = $2)
              AND ($3 IS NULL OR add_time >= $3)
              AND ($4 IS NULL OR add_time <= $4)
              AND is_deleted = false
            ORDER BY id DESC
            LIMIT $5 OFFSET $6
            "#,
            USER_BLACK_COLUMNS
        );

        let list = sqlx::query_as::<_, UserBlackEntity>(&list_query)
            .bind(actor_id)
            .bind(target_id)
            .bind(start_time)
            .bind(end_time)
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await?;

        Ok((total, list))
    }
}

//////// END