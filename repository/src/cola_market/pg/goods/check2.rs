// /check2.rs
// 
// 2026/8/11 08:08 Created.

////////


// repository/src/cola_market/pg/goods/check.rs
// 仓储 - MARKET - pg - 商品 - 状态检查
// 2026/8/11 07:38 Created.

////////

use crate::pg_pool;
use cola_data::cola_market::entity::goods::goods::{GOODS_COLUMNS, GoodsEntity};

////////

/// # [CHECK REPOSITORY] - 检查
/// * `desc`: `检查商品的状态`
pub struct GoodsChangeRepo;

impl GoodsChangeRepo {
    //

    ////////

    /// # 1. [REPOSITORY] - 健康
    pub async fn check_health(id: i64, // 商品 ID
    ) -> Result<u64, sqlx::Error> {
        let pool = pg_pool();
        let query = "SELECT health FROM cola_market.goods WHERE id = $1";

        sqlx::query_scalar::<_, i64>(query)
            .bind(id)
            .fetch_optional(&pool)
            .await
            .map(|res| match res {
                Some(val) => val as u64,
                None => 0,
            })
            .map_err(|e| {
                eprintln!(
                    "[DB ERROR] check_health | SQL: {} | id: {} | err: {:?}",
                    query, id, e
                );
                e
            })
    }

    ////////

    /// # 2. [REPOSITORY] - 状态
    pub async fn check_status(id: i64, // 商品 ID
    ) -> Result<GoodsEntity, sqlx::Error> {
        let pool = pg_pool();
        let query = format!(
            "SELECT {} FROM cola_market.goods WHERE id = $1",
            GOODS_COLUMNS
        );

        let res = sqlx::query_as::<_, GoodsEntity>(&query)
            .bind(id)
            .fetch_optional(&pool)
            .await
            .map_err(|e| {
                eprintln!(
                    "[DB ERROR] check_status | SQL: {} | id: {} | err: {:?}",
                    query, id, e
                );
                e
            })?;

        match res {
            Some(goods) => Ok(goods),
            None => {
                let err = sqlx::Error::RowNotFound;
                eprintln!(
                    "[DB ERROR] check_status | goods not found | SQL: {} | id: {} | err: {:?}",
                    query, id, err
                );
                Err(err)
            }
        }
    }

    ////////

    /// # 3. [REPOSITORY] - 归属
    pub async fn check_owner(
        user_id: i64, // 用户 ID
        id: i64,      // 商品 ID
    ) -> Result<bool, sqlx::Error> {
        let pool = pg_pool();
        let query = "SELECT 1 FROM cola_market.goods WHERE id = $1 AND uid = $2";

        sqlx::query_scalar::<_, i32>(query)
            .bind(id)
            .bind(user_id)
            .fetch_optional(&pool)
            .await
            .map(|res| res.is_some())
            .map_err(|e| {
                eprintln!(
                    "[DB ERROR] check_owner | SQL: {} | id: {} | user_id: {} | err: {:?}",
                    query, id, user_id, e
                );
                e
            })
    }
}

//////// END