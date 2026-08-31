// repository/src/market/pg/goods/delete.rs
// 仓储 - MARKET - pg - 商品 - 删除
// 2026/8/11 07:26 Created.

////////

use crate::pg_pool;
use sqlx::{self, QueryBuilder};

////////

/// # [DELETE REPOSITORY] - 删除
/// * `desc`: `商品逻辑删除仓储` - 逻辑删除
pub struct GoodsDeleteRepo;

impl GoodsDeleteRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 单个删除
    /// * `DESC`: `逻辑删除` - is_deleted = true / deleted_at = now
    pub async fn delete_by_id(id: i64, // 商品 ID
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let query = "UPDATE market.goods SET is_deleted = true, deleted_at = NOW() WHERE id = $1 AND is_deleted = false";

        sqlx::query(query)
            .bind(id)
            .execute(&pool)
            .await
            .map(|res| res.rows_affected())
            .map_err(|e| {
                eprintln!(
                    "[DB ERROR] delete_by_id | SQL: {} | id: {} | err: {:?}",
                    query, id, e
                );
                e
            })
    }

    ////////

    /// # 2. [REPOSITORY] - 批量删除
    /// * `DESC`: `逻辑删除` - is_deleted = true / deleted_at = now
    pub async fn delete_by_ids(ids: &[i64], // 商品 IDs
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        if ids.is_empty() {
            return Ok(0);
        }

        let mut query_builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            "UPDATE market.goods SET is_deleted = true, deleted_at = NOW() WHERE is_deleted = false AND id IN (",
        );

        let mut separated = query_builder.separated(", ");
        for id in ids {
            separated.push_bind(id);
        }
        separated.push_unseparated(")");

        let query = query_builder.build();
        let sql_str = "UPDATE market.goods SET is_deleted = true, deleted_at = NOW() WHERE is_deleted = false AND id = ANY($1)";

        sqlx::query(sql_str)
            .bind(ids)
            .execute(&pool)
            .await
            .map(|res| res.rows_affected())
            .map_err(|e| {
                eprintln!("[DB ERROR] delete_by_ids | ids: {:?} | err: {:?}", ids, e);
                e
            })
    }

    ////////

    /// # 3. [REPOSITORY] - 根据用户ID删除
    /// * `DESC`: `逻辑删除` - is_deleted = true / deleted_at = now
    pub async fn delete_by_user_id(user_id: i64, // 用户 ID
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let query = "UPDATE market.goods SET is_deleted = true, deleted_at = NOW() WHERE uid = $1 AND is_deleted = false";

        sqlx::query(query)
            .bind(user_id)
            .execute(&pool)
            .await
            .map(|res| res.rows_affected())
            .map_err(|e| {
                eprintln!(
                    "[DB ERROR] delete_by_user_id | user_id: {} | err: {:?}",
                    user_id, e
                );
                e
            })
    }
}

//////// END
